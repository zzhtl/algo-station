//! 本地判题队列、Docker 沙箱和结果模型。

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::{Duration, Instant};

use serde::Serialize;
use sqlx::SqlitePool;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

use crate::curriculum::ExerciseDefinition;

const CAPTURE_LIMIT: usize = 64 * 1024;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SubmissionJob {
    pub id: i64,
    pub exercise_slug: String,
    pub lesson_slug: String,
    pub language: String,
    pub contract: String,
    pub source_code: String,
    pub attempts: i64,
}

pub async fn claim_next_submission(
    pool: &SqlitePool,
    worker_id: &str,
    lease_seconds: i64,
) -> Result<Option<SubmissionJob>, sqlx::Error> {
    sqlx::query_as::<_, SubmissionJob>(
        "UPDATE submissions
         SET status = 'running',
             attempts = attempts + 1,
             lease_owner = ?,
             lease_until = datetime('now', '+' || ? || ' seconds'),
             started_at = COALESCE(started_at, datetime('now')),
             updated_at = datetime('now')
         WHERE id = (
             SELECT id FROM submissions
             WHERE attempts < 3
               AND (status = 'queued' OR (status = 'running' AND lease_until < datetime('now')))
             ORDER BY id LIMIT 1
         )
         RETURNING id, exercise_slug, lesson_slug, language, contract, source_code, attempts",
    )
    .bind(worker_id)
    .bind(lease_seconds.max(10))
    .fetch_optional(pool)
    .await
}

pub async fn renew_submission_lease(
    pool: &SqlitePool,
    worker_id: &str,
    submission_id: i64,
    lease_seconds: i64,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE submissions
         SET lease_until = datetime('now', '+' || ? || ' seconds'), updated_at = datetime('now')
         WHERE id = ? AND status = 'running' AND lease_owner = ?",
    )
    .bind(lease_seconds.max(10))
    .bind(submission_id)
    .bind(worker_id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() == 1)
}

pub async fn reap_exhausted_submissions(pool: &SqlitePool) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE submissions
         SET status = 'internal_error',
             result_json = '{\"message\":\"worker lease expired after 3 attempts\"}',
             lease_owner = NULL,
             lease_until = NULL,
             finished_at = datetime('now'),
             updated_at = datetime('now')
         WHERE status = 'running' AND attempts >= 3 AND lease_until < datetime('now')",
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

pub async fn finish_submission(
    pool: &SqlitePool,
    worker_id: &str,
    submission_id: i64,
    outcome: &JudgeOutcome,
) -> Result<bool, anyhow::Error> {
    let result_json = serde_json::to_string(outcome)?;
    let mut transaction = pool.begin().await?;
    let updated = sqlx::query(
        "UPDATE submissions
         SET status = ?, result_json = ?, lease_owner = NULL, lease_until = NULL,
             finished_at = datetime('now'), updated_at = datetime('now')
         WHERE id = ? AND status = 'running' AND lease_owner = ?",
    )
    .bind(outcome.status.as_str())
    .bind(result_json)
    .bind(submission_id)
    .bind(worker_id)
    .execute(&mut *transaction)
    .await?;
    if updated.rows_affected() == 1 && outcome.status == JudgeStatus::Accepted {
        sqlx::query(
            "INSERT INTO learning_events (event_type, lesson_slug, exercise_slug)
             SELECT 'exercise_accepted', lesson_slug, exercise_slug
             FROM submissions WHERE id = ?",
        )
        .bind(submission_id)
        .execute(&mut *transaction)
        .await?;
    }
    transaction.commit().await?;
    Ok(updated.rows_affected() == 1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum JudgeStatus {
    Accepted,
    WrongAnswer,
    CompileError,
    RuntimeError,
    TimeLimit,
    MemoryLimit,
    InternalError,
}

impl JudgeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::WrongAnswer => "wrong_answer",
            Self::CompileError => "compile_error",
            Self::RuntimeError => "runtime_error",
            Self::TimeLimit => "time_limit",
            Self::MemoryLimit => "memory_limit",
            Self::InternalError => "internal_error",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct CaseResult {
    pub name: String,
    pub visibility: String,
    pub status: JudgeStatus,
    pub duration_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct JudgeOutcome {
    pub status: JudgeStatus,
    pub message: String,
    pub compile_output: String,
    pub cases: Vec<CaseResult>,
    pub duration_ms: u64,
}

impl JudgeOutcome {
    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            status: JudgeStatus::InternalError,
            message: message.into(),
            compile_output: String::new(),
            cases: Vec::new(),
            duration_ms: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DockerJudge {
    pub docker_bin: String,
    pub go_image: String,
    pub rust_image: String,
}

impl Default for DockerJudge {
    fn default() -> Self {
        Self {
            docker_bin: "docker".to_owned(),
            go_image: "algo-station-go-runner:1.0".to_owned(),
            rust_image: "algo-station-rust-runner:1.0".to_owned(),
        }
    }
}

impl DockerJudge {
    pub async fn verify(&self, require_rootless: bool) -> anyhow::Result<()> {
        let output = Command::new(&self.docker_bin)
            .args(["info", "--format", "{{json .SecurityOptions}}"])
            .output()
            .await?;
        if !output.status.success() {
            anyhow::bail!(
                "Docker unavailable: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        let security_options = String::from_utf8_lossy(&output.stdout);
        if require_rootless && !security_options.to_ascii_lowercase().contains("rootless") {
            anyhow::bail!("Judge Worker requires rootless Docker by default");
        }
        for image in [&self.go_image, &self.rust_image] {
            let status = Command::new(&self.docker_bin)
                .args(["image", "inspect", image])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .await?;
            if !status.success() {
                anyhow::bail!("runner image is missing: {image}");
            }
        }
        Ok(())
    }

    pub async fn judge(
        &self,
        job: &SubmissionJob,
        exercise: &ExerciseDefinition,
    ) -> JudgeOutcome {
        let started = Instant::now();
        let workspace = match Workspace::create(job.id) {
            Ok(workspace) => workspace,
            Err(error) => return JudgeOutcome::internal(format!("create workspace: {error}")),
        };
        let (image, compile_program, compile_args) = match prepare_sources(
            workspace.path(),
            &job.language,
            &job.contract,
            &job.source_code,
        ) {
            Ok((program, args)) => {
                let image = if job.language == "go" {
                    self.go_image.as_str()
                } else {
                    self.rust_image.as_str()
                };
                (image, program, args)
            }
            Err(error) => return JudgeOutcome::internal(format!("prepare source: {error}")),
        };
        let compile_name = format!("algo-station-{}-compile", job.id);
        let compile = self
            .run_container(
                &compile_name,
                image,
                workspace.path(),
                exercise.limits.memory_mb.max(512),
                &compile_program,
                &compile_args,
                "",
                Duration::from_millis(exercise.limits.compile_ms),
                CAPTURE_LIMIT,
            )
            .await;
        let compile = match compile {
            Ok(output) => output,
            Err(error) => {
                return JudgeOutcome {
                    status: JudgeStatus::InternalError,
                    message: error.to_string(),
                    compile_output: String::new(),
                    cases: Vec::new(),
                    duration_ms: started.elapsed().as_millis() as u64,
                }
            }
        };
        let compile_output = combined_output(&compile);
        if compile.timed_out {
            return JudgeOutcome {
                status: JudgeStatus::CompileError,
                message: "compilation timed out".to_owned(),
                compile_output,
                cases: Vec::new(),
                duration_ms: started.elapsed().as_millis() as u64,
            };
        }
        if compile.exit_code != Some(0) {
            return JudgeOutcome {
                status: JudgeStatus::CompileError,
                message: "compilation failed".to_owned(),
                compile_output,
                cases: Vec::new(),
                duration_ms: started.elapsed().as_millis() as u64,
            };
        }

        let mut cases = Vec::new();
        let total_deadline = Duration::from_millis(exercise.limits.total_ms);
        for (index, test_case) in exercise.cases.iter().enumerate() {
            let elapsed = started.elapsed();
            if elapsed >= total_deadline {
                cases.push(CaseResult {
                    name: test_case.name.clone(),
                    visibility: test_case.visibility.clone(),
                    status: JudgeStatus::TimeLimit,
                    duration_ms: 0,
                    actual: None,
                    message: Some("total time limit exceeded".to_owned()),
                });
                return final_outcome(
                    JudgeStatus::TimeLimit,
                    "total time limit exceeded",
                    compile_output,
                    cases,
                    started,
                );
            }
            let case_limit = Duration::from_millis(exercise.limits.case_ms)
                .min(total_deadline.saturating_sub(elapsed));
            let case_started = Instant::now();
            let container_name = format!("algo-station-{}-case-{}", job.id, index);
            let output = self
                .run_container(
                    &container_name,
                    image,
                    workspace.path(),
                    exercise.limits.memory_mb,
                    "/workspace/program",
                    &[],
                    &test_case.input,
                    case_limit,
                    exercise.limits.output_kb as usize * 1024,
                )
                .await;
            let output = match output {
                Ok(output) => output,
                Err(error) => {
                    return final_outcome(
                        JudgeStatus::InternalError,
                        &error.to_string(),
                        compile_output,
                        cases,
                        started,
                    )
                }
            };
            let duration_ms = case_started.elapsed().as_millis() as u64;
            let public = test_case.visibility == "public";
            let actual = String::from_utf8_lossy(&output.stdout).into_owned();
            let (status, message) = if output.timed_out {
                (JudgeStatus::TimeLimit, Some("case timed out".to_owned()))
            } else if output.output_limit_exceeded {
                (
                    JudgeStatus::RuntimeError,
                    Some("output limit exceeded".to_owned()),
                )
            } else if output.exit_code == Some(137) {
                (
                    JudgeStatus::MemoryLimit,
                    Some("memory limit exceeded".to_owned()),
                )
            } else if output.exit_code != Some(0) {
                (
                    JudgeStatus::RuntimeError,
                    Some(truncate_text(&String::from_utf8_lossy(&output.stderr), 2048)),
                )
            } else if !outputs_match(&actual, &test_case.expected) {
                (JudgeStatus::WrongAnswer, Some("output mismatch".to_owned()))
            } else {
                (JudgeStatus::Accepted, None)
            };
            cases.push(CaseResult {
                name: test_case.name.clone(),
                visibility: test_case.visibility.clone(),
                status,
                duration_ms,
                actual: public.then(|| truncate_text(&actual, 4096)),
                message,
            });
            if status != JudgeStatus::Accepted {
                return final_outcome(
                    status,
                    status_message(status),
                    compile_output,
                    cases,
                    started,
                );
            }
        }
        final_outcome(
            JudgeStatus::Accepted,
            "all cases passed",
            compile_output,
            cases,
            started,
        )
    }

    #[allow(clippy::too_many_arguments)]
    async fn run_container(
        &self,
        container_name: &str,
        image: &str,
        workspace: &Path,
        memory_mb: u64,
        program: &str,
        program_args: &[String],
        input: &str,
        timeout: Duration,
        output_limit: usize,
    ) -> anyhow::Result<ProcessOutput> {
        let mut command = Command::new(&self.docker_bin);
        command.args(docker_sandbox_args(workspace, memory_mb));
        command.args(["--name", container_name]);
        command.arg(image);
        command.arg(program);
        command.args(program_args);
        let result = run_command(&mut command, input, timeout, output_limit).await;
        if result.as_ref().is_ok_and(|output| output.timed_out) || result.is_err() {
            let _ = Command::new(&self.docker_bin)
                .args(["rm", "-f", container_name])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()
                .await;
        }
        result
    }
}

pub fn docker_sandbox_args(workspace: &Path, memory_mb: u64) -> Vec<String> {
    let memory_mb = memory_mb.clamp(32, 512);
    let tmpfs_mb = (memory_mb / 2).clamp(64, 256);
    vec![
        "run".to_owned(),
        "--rm".to_owned(),
        "--interactive".to_owned(),
        "--network".to_owned(),
        "none".to_owned(),
        "--read-only".to_owned(),
        "--cap-drop".to_owned(),
        "ALL".to_owned(),
        "--security-opt".to_owned(),
        "no-new-privileges".to_owned(),
        "--pids-limit".to_owned(),
        "64".to_owned(),
        "--memory".to_owned(),
        format!("{memory_mb}m"),
        "--memory-swap".to_owned(),
        format!("{memory_mb}m"),
        "--cpus".to_owned(),
        "1".to_owned(),
        "--user".to_owned(),
        "65532:65532".to_owned(),
        "--env".to_owned(),
        "HOME=/tmp".to_owned(),
        "--env".to_owned(),
        "GOCACHE=/tmp/go-cache".to_owned(),
        "--tmpfs".to_owned(),
        format!("/tmp:rw,nosuid,size={tmpfs_mb}m,mode=1777"),
        "--volume".to_owned(),
        format!("{}:/workspace:rw", workspace.display()),
        "--workdir".to_owned(),
        "/workspace".to_owned(),
    ]
}

pub fn outputs_match(actual: &str, expected: &str) -> bool {
    normalize_output(actual) == normalize_output(expected)
}

fn normalize_output(value: &str) -> String {
    value
        .replace("\r\n", "\n")
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end()
        .to_owned()
}

fn prepare_sources(
    workspace: &Path,
    language: &str,
    contract: &str,
    source: &str,
) -> anyhow::Result<(String, Vec<String>)> {
    match (language, contract) {
        ("go", "stdio") => {
            std::fs::write(workspace.join("main.go"), source)?;
            Ok((
                "go".to_owned(),
                vec![
                    "build".to_owned(),
                    "-trimpath".to_owned(),
                    "-o".to_owned(),
                    "/workspace/program".to_owned(),
                    "/workspace/main.go".to_owned(),
                ],
            ))
        }
        ("go", "function") => {
            std::fs::write(workspace.join("solution.go"), source)?;
            std::fs::write(
                workspace.join("runner.go"),
                "package main\n\nimport (\"fmt\"; \"io\"; \"os\")\n\nfunc main() { input, _ := io.ReadAll(os.Stdin); fmt.Print(Solve(string(input))) }\n",
            )?;
            Ok((
                "go".to_owned(),
                vec![
                    "build".to_owned(),
                    "-trimpath".to_owned(),
                    "-o".to_owned(),
                    "/workspace/program".to_owned(),
                    "/workspace/solution.go".to_owned(),
                    "/workspace/runner.go".to_owned(),
                ],
            ))
        }
        ("rust", "stdio") => {
            std::fs::write(workspace.join("main.rs"), source)?;
            Ok((
                "rustc".to_owned(),
                rust_compile_args("/workspace/main.rs"),
            ))
        }
        ("rust", "function") => {
            std::fs::write(workspace.join("solution.rs"), source)?;
            std::fs::write(
                workspace.join("main.rs"),
                "mod solution;\nuse std::io::{self, Read};\nfn main() { let mut input = String::new(); io::stdin().read_to_string(&mut input).expect(\"read stdin\"); print!(\"{}\", solution::solve(&input)); }\n",
            )?;
            Ok((
                "rustc".to_owned(),
                rust_compile_args("/workspace/main.rs"),
            ))
        }
        _ => anyhow::bail!("unsupported language/contract: {language}/{contract}"),
    }
}

fn rust_compile_args(source: &str) -> Vec<String> {
    vec![
        source.to_owned(),
        "--edition=2024".to_owned(),
        "-O".to_owned(),
        "-o".to_owned(),
        "/workspace/program".to_owned(),
    ]
}

struct Workspace {
    path: PathBuf,
}

impl Workspace {
    fn create(submission_id: i64) -> std::io::Result<Self> {
        let path = std::env::temp_dir().join(format!(
            "algo-station-judge-{}-{}",
            submission_id,
            std::process::id()
        ));
        if path.exists() {
            std::fs::remove_dir_all(&path)?;
        }
        std::fs::create_dir(&path)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            // bind mount 内的编译器固定以 65532 运行，只开放这一个隔离任务目录的写权限。
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o777))?;
        }
        Ok(Self { path })
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for Workspace {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.path);
    }
}

#[derive(Debug)]
struct ProcessOutput {
    exit_code: Option<i32>,
    stdout: Vec<u8>,
    stderr: Vec<u8>,
    timed_out: bool,
    output_limit_exceeded: bool,
}

async fn run_command(
    command: &mut Command,
    input: &str,
    timeout: Duration,
    output_limit: usize,
) -> anyhow::Result<ProcessOutput> {
    let mut child = command
        .kill_on_drop(true)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes()).await?;
        drop(stdin);
    }
    let stdout = child.stdout.take().expect("stdout piped");
    let stderr = child.stderr.take().expect("stderr piped");
    let stdout_task = tokio::spawn(async move {
        let mut stdout = stdout.take(output_limit as u64 + 1);
        let mut stdout_bytes = Vec::new();
        stdout.read_to_end(&mut stdout_bytes).await?;
        Ok::<_, std::io::Error>(stdout_bytes)
    });
    let stderr_task = tokio::spawn(async move {
        let mut stderr = stderr.take(CAPTURE_LIMIT as u64 + 1);
        let mut stderr_bytes = Vec::new();
        stderr.read_to_end(&mut stderr_bytes).await?;
        Ok::<_, std::io::Error>(stderr_bytes)
    });
    match tokio::time::timeout(timeout, child.wait()).await {
        Ok(status) => {
            let exit_code = status?.code();
            let stdout = stdout_task.await??;
            let stderr = stderr_task.await??;
            Ok(ProcessOutput {
                exit_code,
                output_limit_exceeded: stdout.len() > output_limit,
                stdout,
                stderr,
                timed_out: false,
            })
        }
        Err(_) => {
            let _ = child.kill().await;
            let _ = child.wait().await;
            stdout_task.abort();
            stderr_task.abort();
            Ok(ProcessOutput {
                exit_code: None,
                stdout: Vec::new(),
                stderr: Vec::new(),
                timed_out: true,
                output_limit_exceeded: false,
            })
        }
    }
}

fn combined_output(output: &ProcessOutput) -> String {
    let mut value = String::from_utf8_lossy(&output.stdout).into_owned();
    if !value.is_empty() && !output.stderr.is_empty() {
        value.push('\n');
    }
    value.push_str(&String::from_utf8_lossy(&output.stderr));
    truncate_text(&value, CAPTURE_LIMIT)
}

fn truncate_text(value: &str, limit: usize) -> String {
    if value.len() <= limit {
        return value.to_owned();
    }
    let mut end = limit;
    while !value.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}\n… output truncated", &value[..end])
}

fn final_outcome(
    status: JudgeStatus,
    message: &str,
    compile_output: String,
    cases: Vec<CaseResult>,
    started: Instant,
) -> JudgeOutcome {
    JudgeOutcome {
        status,
        message: message.to_owned(),
        compile_output,
        cases,
        duration_ms: started.elapsed().as_millis() as u64,
    }
}

fn status_message(status: JudgeStatus) -> &'static str {
    match status {
        JudgeStatus::Accepted => "all cases passed",
        JudgeStatus::WrongAnswer => "answer did not match expected output",
        JudgeStatus::CompileError => "compilation failed",
        JudgeStatus::RuntimeError => "program exited abnormally",
        JudgeStatus::TimeLimit => "time limit exceeded",
        JudgeStatus::MemoryLimit => "memory limit exceeded",
        JudgeStatus::InternalError => "judge internal error",
    }
}

#[cfg(all(test, unix))]
mod tests {
    use std::os::unix::fs::PermissionsExt;

    use super::Workspace;

    #[test]
    fn container_user_can_write_the_isolated_workspace() {
        let submission_id = -(std::process::id() as i64);
        let workspace = Workspace::create(submission_id).expect("workspace must be created");
        let mode = std::fs::metadata(workspace.path())
            .expect("workspace metadata")
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o777);
    }
}

use algo_station::curriculum::{ExerciseCase, ExerciseDefinition, ExerciseLimits};
use algo_station::judge::{DockerJudge, JudgeStatus, SubmissionJob};

fn exercise(case_ms: u64, output_kb: u64) -> ExerciseDefinition {
    ExerciseDefinition {
        slug: "docker-e2e".to_owned(),
        problem_id: 0,
        lesson_slug: "docker-e2e-lesson".to_owned(),
        title: "Docker E2E fixture".to_owned(),
        difficulty: "Easy".to_owned(),
        summary: "Controlled fixture for the local judge".to_owned(),
        starters: Vec::new(),
        cases: vec![ExerciseCase {
            name: "echo".to_owned(),
            visibility: "public".to_owned(),
            input: "hello judge\n".to_owned(),
            expected: "hello judge\n".to_owned(),
        }],
        limits: ExerciseLimits {
            compile_ms: 20_000,
            case_ms,
            total_ms: 30_000,
            memory_mb: 256,
            output_kb,
        },
    }
}

fn submission(id: i64, language: &str, contract: &str, source_code: &str) -> SubmissionJob {
    SubmissionJob {
        id,
        exercise_slug: "docker-e2e".to_owned(),
        lesson_slug: "docker-e2e-lesson".to_owned(),
        language: language.to_owned(),
        contract: contract.to_owned(),
        source_code: source_code.to_owned(),
        attempts: 1,
    }
}

#[tokio::test]
#[ignore = "requires the pinned Docker runner images"]
async fn real_docker_judge_covers_success_and_failure_outcomes() {
    let judge = DockerJudge::default();
    // Sources below are fixed test fixtures. Production still requires rootless Docker by default.
    judge.verify(false).await.expect("Docker runner images");

    let successful = [
        (
            91_001,
            "go",
            "function",
            "package main\nfunc Solve(input string) string { return input }\n",
        ),
        (
            91_002,
            "go",
            "stdio",
            "package main\nimport (\"io\"; \"os\")\nfunc main() { _, _ = io.Copy(os.Stdout, os.Stdin) }\n",
        ),
        (
            91_003,
            "rust",
            "function",
            "pub fn solve(input: &str) -> String { input.to_owned() }\n",
        ),
        (
            91_004,
            "rust",
            "stdio",
            "use std::io::{self, Read};\nfn main() { let mut s = String::new(); io::stdin().read_to_string(&mut s).unwrap(); print!(\"{}\", s); }\n",
        ),
    ];
    for (id, language, contract, source) in successful {
        let outcome = judge
            .judge(&submission(id, language, contract, source), &exercise(2_000, 64))
            .await;
        assert_eq!(
            outcome.status,
            JudgeStatus::Accepted,
            "{language}/{contract}: {}\n{}\n{:?}",
            outcome.message,
            outcome.compile_output,
            outcome.cases
        );
    }

    let wrong = judge
        .judge(
            &submission(
                91_005,
                "go",
                "function",
                "package main\nfunc Solve(input string) string { return \"wrong\" }\n",
            ),
            &exercise(2_000, 64),
        )
        .await;
    assert_eq!(wrong.status, JudgeStatus::WrongAnswer);

    let compile_error = judge
        .judge(
            &submission(91_006, "rust", "stdio", "this is not valid Rust"),
            &exercise(2_000, 64),
        )
        .await;
    assert_eq!(compile_error.status, JudgeStatus::CompileError);
    assert!(!compile_error.compile_output.is_empty());

    let timeout = judge
        .judge(
            &submission(91_007, "go", "stdio", "package main\nfunc main() { for {} }\n"),
            &exercise(300, 64),
        )
        .await;
    assert_eq!(timeout.status, JudgeStatus::TimeLimit);

    let output_limit = judge
        .judge(
            &submission(
                91_008,
                "go",
                "stdio",
                "package main\nimport (\"fmt\"; \"strings\")\nfunc main() { fmt.Print(strings.Repeat(\"x\", 2048)) }\n",
            ),
            &exercise(2_000, 1),
        )
        .await;
    assert_eq!(output_limit.status, JudgeStatus::RuntimeError);
    assert_eq!(output_limit.cases[0].message.as_deref(), Some("output limit exceeded"));
}

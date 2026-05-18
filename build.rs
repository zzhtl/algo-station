use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    // 允许通过环境变量跳过前端构建（开发期使用）
    if std::env::var("SKIP_FRONTEND_BUILD").ok().as_deref() == Some("1") {
        println!("cargo:warning=SKIP_FRONTEND_BUILD=1, 跳过前端构建");
        // 仍需创建占位目录，避免 rust-embed 找不到目录而编译失败
        ensure_placeholder_build_dir();
        return;
    }

    let manifest_dir = PathBuf::from(env("CARGO_MANIFEST_DIR"));
    let frontend_dir = manifest_dir.join("frontend");

    // 嵌入到 binary 的原创题解；目录变化需触发重编以让 rust-embed 重新打包。
    rerun_if_changed(&manifest_dir.join("content"));

    if !frontend_dir.exists() {
        panic!("找不到 frontend 目录: {}", frontend_dir.display());
    }

    // 监听前端关键文件变化
    rerun_if_changed(&frontend_dir.join("src"));
    rerun_if_changed(&frontend_dir.join("static"));
    rerun_if_changed(&frontend_dir.join("package.json"));
    rerun_if_changed(&frontend_dir.join("svelte.config.js"));
    rerun_if_changed(&frontend_dir.join("vite.config.ts"));
    rerun_if_changed(&frontend_dir.join("tailwind.config.js"));
    rerun_if_changed(&frontend_dir.join("postcss.config.js"));
    rerun_if_changed(&frontend_dir.join("tsconfig.json"));
    println!("cargo:rerun-if-env-changed=SKIP_FRONTEND_BUILD");

    let bun = which("bun").unwrap_or_else(|| {
        panic!("未找到 bun 可执行文件，请先安装 bun (https://bun.sh) 或设置 SKIP_FRONTEND_BUILD=1 跳过前端构建");
    });

    if !frontend_dir.join("node_modules").exists() {
        eprintln!("[build.rs] 执行 bun install...");
        let args: &[&str] = &["install"];
        run(&bun, args, &frontend_dir);
    }

    eprintln!("[build.rs] 执行 bun run build...");
    let args: &[&str] = &["run", "build"];
    run(&bun, args, &frontend_dir);

    let build_dir = frontend_dir.join("build");
    if !build_dir.exists() {
        panic!(
            "前端构建未产生 build/ 目录: {}",
            build_dir.display()
        );
    }
}

fn run(program: &Path, args: &[&str], cwd: &Path) {
    let status = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .status()
        .unwrap_or_else(|e| panic!("无法启动 {}: {}", program.display(), e));
    if !status.success() {
        panic!(
            "命令失败: {} {:?} (cwd: {})",
            program.display(),
            args,
            cwd.display()
        );
    }
}

fn rerun_if_changed(p: &Path) {
    println!("cargo:rerun-if-changed={}", p.display());
}

fn env(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("缺少环境变量 {}", key))
}

fn which(cmd: &str) -> Option<PathBuf> {
    let path = std::env::var_os("PATH")?;
    for p in std::env::split_paths(&path) {
        let candidate = p.join(cmd);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

fn ensure_placeholder_build_dir() {
    let manifest_dir = PathBuf::from(env("CARGO_MANIFEST_DIR"));
    let build_dir = manifest_dir.join("frontend").join("build");
    if !build_dir.exists() {
        let _ = std::fs::create_dir_all(&build_dir);
        // 写一个占位 index.html，避免空目录在某些平台上的尴尬
        let placeholder = build_dir.join("index.html");
        if !placeholder.exists() {
            let _ = std::fs::write(
                &placeholder,
                "<!doctype html><meta charset=utf-8><title>frontend not built</title><body>前端尚未构建，请运行 <code>cargo build --release</code> (不要设置 SKIP_FRONTEND_BUILD)。</body>",
            );
        }
    }
}

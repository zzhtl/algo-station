default:
    @just --list

# 后端：调试模式运行 HTTP 服务
backend:
    SKIP_FRONTEND_BUILD=1 cargo run

# 后端：发布模式运行（会顺带构建前端并嵌入二进制）
backend-release:
    cargo run --release

# 整个站点：编译出单个可执行文件 target/release/algo-station
build:
    cargo build --release

# 扫描本地 doocs/leetcode 并重建 SQLite 索引
import:
    SKIP_FRONTEND_BUILD=1 cargo run --release --bin scrape

# 前端：开发服务器（热更新）
frontend:
    cd frontend && bun run dev

# 前端：依赖安装
install:
    cd frontend && bun install

# 确定性重建 12 阶段课程目录
content:
    python3 scripts/generate_curriculum.py

# 独立判题 Worker（与主站共享 DATABASE_URL）
judge:
    SKIP_FRONTEND_BUILD=1 cargo run --bin judge_worker

# 构建固定版本的 Go / Rust 判题镜像
runner-images:
    docker build -t algo-station-go-runner:1.0 runner/go
    docker build -t algo-station-rust-runner:1.0 runner/rust

# 一次性把后端 + 前端开发模式都拉起 (用 trap 在 Ctrl-C 时一起退出)
dev:
    #!/usr/bin/env bash
    set -m
    SKIP_FRONTEND_BUILD=1 cargo run &
    BACK=$!
    (cd frontend && bun run dev) &
    FRONT=$!
    trap "kill $BACK $FRONT 2>/dev/null" EXIT INT TERM
    wait

# 后端类型检查
check:
    SKIP_FRONTEND_BUILD=1 cargo check

# 前端类型检查
check-fe:
    cd frontend && bun run check

# 后端、课程与前端的完整快速回归
check-all:
    SKIP_FRONTEND_BUILD=1 cargo test --all-targets
    cd frontend && bun run check
    cd frontend && bun run test
    cd frontend && bun run build

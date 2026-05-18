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

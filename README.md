# Algo Station

Algo Station 是一个离线优先、单用户自托管的渐进式算法学习站。它把课程、动态图解、在线判题和间隔复习串成一条学习路径，同时保留完整题库与文章资料库供随时查阅。

## 主要能力

- 12 个递进阶段、100 节精选课程、150 道配套判题练习。
- 25 套算法专属动态图，逐帧展示状态、指针、调用栈与记忆提示。
- Go / Rust 双语言，每题同时支持函数模式和标准输入输出模式。
- 服务端根据“小测 ≥ 80 分 + 核心题 Accepted + 动图播放完成”推导课程完成状态。
- 完成课程后按 1、3、7、14、30 天安排复习，并根据“忘记 / 模糊 / 清楚”调整间隔。
- 每日默认 60 分钟学习计划、轻量连续天数、进度导入导出。
- 保留 344 篇站内文章、完整 LeetCode 本地索引、全文搜索、标签与收藏。

## 架构

```text
Browser (SvelteKit SPA)
          │ HTTP /api
          ▼
algo-station (axum)
  ├─ bundled curriculum / articles / frontend
  ├─ SQLite: progress, reviews, drafts, submissions
  └─ submission queue
          │ lease + heartbeat
          ▼
judge_worker
  └─ rootless Docker
       ├─ Go runner image
       └─ Rust runner image
```

主站即使没有启动 Judge Worker 也可正常阅读、播放图解、答题和保存代码。提交会留在 SQLite 队列，Worker 上线后继续处理。

关键设计见：

- [渐进式课程模型](docs/adr/0001-guided-curriculum.md)
- [SQLite 判题队列](docs/adr/0002-sqlite-judge-queue.md)
- [rootless Docker 沙箱](docs/adr/0003-rootless-docker-sandbox.md)
- [OpenAPI 3.1](docs/openapi.yaml)
- [判题运维手册](docs/judge-runbook.md)

## 环境要求

- Rust 1.85+（项目使用 Rust 2024 edition）
- bun 1.3+
- Python 3.10+（仅用于确定性生成课程目录）
- rootless Docker（仅在线判题需要）
- 本地 `doocs/leetcode` 数据（完整题库与题面需要）

release 二进制内已包含前端、课程目录和站内文章；运行主站不需要 bun 或 Python。

## 快速开始

安装前端依赖并校验课程内容：

```bash
cd frontend
bun install
cd ..
python3 scripts/generate_curriculum.py
```

启动主站：

```bash
SKIP_FRONTEND_BUILD=1 cargo run --bin algo-station
```

默认仅监听 `127.0.0.1:8928`，访问 <http://127.0.0.1:8928>。

前端热更新模式：

```bash
cd frontend
bun run dev
```

Vite 会把 `/api` 代理到 `BACKEND_URL`，默认是 `http://127.0.0.1:8928`。

## 启用在线判题

Judge Worker 默认拒绝连接 rootful Docker。先按 Docker 官方方式配置当前用户的 rootless daemon，并确认：

```bash
docker info --format '{{json .SecurityOptions}}'
```

输出应包含 `rootless`。然后构建固定版本的 runner 镜像：

```bash
docker build -t algo-station-go-runner:1.0 runner/go
docker build -t algo-station-rust-runner:1.0 runner/rust
```

主站和 Worker 必须使用同一个 SQLite 文件：

```bash
DATABASE_URL='sqlite://data/algo.db?mode=rwc' \
  SKIP_FRONTEND_BUILD=1 cargo run --bin judge_worker
```

Worker 对每次编译和运行都设置：

- `--network none`
- `--read-only`
- `--cap-drop ALL`
- `no-new-privileges`
- 非 root 用户 `65532:65532`
- CPU、内存、进程数、时间和输出大小限制

不要把 Judge Worker 暴露为网络服务；它只通过 SQLite 队列与主站协作。详细排障见 [docs/judge-runbook.md](docs/judge-runbook.md)。

## 准备完整题库

推荐把 `doocs/leetcode` 放到 `data/leetcode`：

```bash
git clone --depth=1 https://github.com/doocs/leetcode data/leetcode
./scripts/cleanup_leetcode.sh data/leetcode
SKIP_FRONTEND_BUILD=1 cargo run --release --bin scrape -- --repo data/leetcode
```

如果 `problems` 表为空，主站启动时也会尝试从 `LEETCODE_REPO_DIR` 建立索引。课程与 150 道精选练习的元数据已编译进二进制；缺少本地题库主要影响完整题面、参考解法和资料库题目详情。

## 学习规则

课程前置关系是软约束：界面会提示缺失的基础，但不会锁住课程。

一节课程由服务端在以下条件同时满足时标记完成：

1. 小测历史最高分至少 80 分；
2. 所有核心练习至少有一次 `accepted`；
3. 课程有动态图时，动态图已走到最后一帧。

旧版训练记录会在首次启动后迁移：`reviewed` 视作已完成，`learned` / `practiced` 视作进行中，原笔记会保留。`/api/progress/import` 同时接受旧版 v1 和新版 v2 导出文件。

## 构建与验证

后端检查与测试：

```bash
SKIP_FRONTEND_BUILD=1 cargo check --all-targets
SKIP_FRONTEND_BUILD=1 cargo test --all-targets
```

前端检查、测试与构建：

```bash
cd frontend
bun run check
bun run test
bun run build
```

发布构建：

```bash
cargo build --release --bin algo-station
```

`build.rs` 会用 bun 构建前端并嵌入主二进制。`SKIP_FRONTEND_BUILD=1` 只适合后端开发检查，不应用于正式发布。

也可以使用 Justfile：

```bash
just check-all
just backend
just frontend
just runner-images
just judge
```

## 环境变量

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `HOST` | `127.0.0.1` | 主站监听地址；只有明确需要内网访问时才改为其他地址 |
| `PORT` | `8928` | 主站监听端口 |
| `DATABASE_URL` | `sqlite://data/algo.db?mode=rwc` | 主站与 Worker 必须指向同一个数据库 |
| `LEETCODE_REPO_DIR` | `data/leetcode` | 本地 doocs/leetcode 根目录 |
| `BACKEND_URL` | `http://127.0.0.1:8928` | Vite 开发代理目标 |
| `RUST_LOG` | `info` | Rust 日志过滤 |
| `SKIP_FRONTEND_BUILD` | 未设置 | 设为 `1` 可在后端开发时跳过前端构建 |
| `JUDGE_WORKER_ID` | 主机名 + PID | Worker 唯一标识 |
| `DOCKER_BIN` | `docker` | Docker CLI 路径 |
| `JUDGE_GO_IMAGE` | `algo-station-go-runner:1.0` | Go runner 镜像 |
| `JUDGE_RUST_IMAGE` | `algo-station-rust-runner:1.0` | Rust runner 镜像 |
| `JUDGE_ALLOW_ROOTFUL` | 未设置 | 仅紧急受控环境可设为 `1`；不推荐 |

## 目录结构

```text
algo-station/
├── content/
│   ├── articles/                 站内 Markdown 文章
│   └── curriculum/catalog.json  生成后的课程目录
├── docs/                         OpenAPI、ADR、运维文档
├── frontend/                     SvelteKit 前端
├── migrations/                   SQLite 迁移
├── runner/{go,rust}/             固定版本判题镜像
├── scripts/generate_curriculum.py
├── src/
│   ├── main.rs                   HTTP 主站
│   ├── bin/judge_worker.rs       独立判题 Worker
│   ├── curriculum.rs             内容校验
│   ├── learning.rs               完成与复习领域规则
│   ├── judge.rs                  队列、沙箱与结果模型
│   └── routes/                   HTTP API
└── tests/                        课程、学习、API、队列契约测试
```

`content/curriculum/catalog.json` 是确定性生成物。修改课程选择策略或文章 frontmatter 后运行 `python3 scripts/generate_curriculum.py`，不要手工维护大段目录 JSON。

## 数据与备份

个人学习数据都在 `data/algo.db`。备份前可停止主站和 Worker，然后复制数据库；在线备份请使用 SQLite backup API 或 `.backup`，不要只复制正在写入的 WAL 主文件。

界面中的进度导出适合跨设备迁移学习状态和代码草稿，不替代完整数据库备份。迁移、回滚和队列恢复步骤见 [docs/rollback.md](docs/rollback.md)。

## 内容来源与许可

| 内容 | 来源 | 许可 |
| --- | --- | --- |
| LeetCode 题面、解法说明、参考代码 | [doocs/leetcode](https://github.com/doocs/leetcode) | CC-BY-SA 4.0 |
| 站内文章 `content/articles/*.md` | 项目作者 | 见 LICENSE |
| 后端与前端代码 | 项目作者 | MIT |

来自 doocs/leetcode 的内容按 CC-BY-SA 4.0 要求保留来源署名。

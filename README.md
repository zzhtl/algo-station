# Algo Station

Algo Station 是一个离线优先的自托管算法学习站。后端使用 Rust、axum、SQLite 和 FTS5，前端使用 SvelteKit 与 Svelte 5；发布构建会把前端静态资源和原创题解一起嵌入主二进制。

## 功能

- LeetCode 题库浏览与全文搜索，支持题号、英文标题、中文标题和标签检索。
- 题目详情页展示题面、解法说明和参考代码。
- 200 篇站内原创训练题解，覆盖数组字符串、二分、链表、栈队列、二叉树、回溯、动态规划、图论、贪心、哈希前缀、堆、数学位运算、设计数据结构等专题。
- Markdown 渲染支持 Mermaid、KaTeX、Shiki 代码高亮和暗色主题。
- 单个 Rust 服务同时提供 API 和前端页面，适合本机离线使用或内网部署。

## 内容来源与许可

| 内容 | 来源 | 许可 |
| --- | --- | --- |
| LeetCode 题面、解法说明、参考代码 | [doocs/leetcode](https://github.com/doocs/leetcode) | CC-BY-SA 4.0 |
| 站内原创题解 `content/articles/*.md` | 项目作者 | 见 LICENSE |
| 后端与前端代码 | 项目作者 | MIT |

来自 doocs/leetcode 的内容按 CC-BY-SA 4.0 要求保留来源署名，并以兼容许可再发布。

## 目录结构

```text
algo-station/
├── Cargo.toml              Rust 包与二进制配置
├── build.rs                release 构建时编译前端并嵌入静态资源
├── content/articles/       站内原创 Markdown 题解
├── data/
│   ├── leetcode/           本地 doocs/leetcode 数据源
│   └── algo.db             运行时 SQLite 数据库
├── frontend/               SvelteKit 前端
├── migrations/             SQLx 数据库迁移
├── scripts/
│   └── cleanup_leetcode.sh doocs/leetcode 瘦身脚本
└── src/
    ├── main.rs             HTTP 服务入口
    ├── bin/scrape.rs       扫描本地 doocs/leetcode 并建立索引
    ├── problem_seed.rs     题库索引解析与写库
    ├── articles_seed.rs    原创题解嵌入与索引
    └── routes/             API 路由
```

## 环境要求

- Rust 1.79 或更高版本
- bun，用于前端安装依赖、检查和构建
- 本地 doocs/leetcode 仓库，用于离线题库数据

运行已构建好的 release 二进制时不需要 bun；只有构建或前端开发时需要。

## 准备题库数据

首次使用需要准备本地 doocs/leetcode 数据源。推荐放在项目的 `data/leetcode` 目录。

```bash
git clone --depth=1 https://github.com/doocs/leetcode data/leetcode
./scripts/cleanup_leetcode.sh data/leetcode
```

`cleanup_leetcode.sh` 会保留题目 README 和必要结构，减少本地占用。已经有本地仓库时，也可以通过 `LEETCODE_REPO_DIR` 指向它。

然后建立 SQLite 索引：

```bash
SKIP_FRONTEND_BUILD=1 cargo run --release --bin scrape -- --repo data/leetcode
```

如果数据库为空，主服务启动时也会尝试从 `data/leetcode` 自动建立索引。显式运行 `scrape` 更适合初始化、更新题库或排查数据问题。

## 构建

发布构建：

```bash
cargo build --release
```

构建脚本会自动在 `frontend/` 下执行 `bun install` 和 `bun run build`，并把前端产物嵌入二进制。主二进制产物为：

```text
target/release/algo-station
```

开发期只检查后端时可以跳过前端构建：

```bash
SKIP_FRONTEND_BUILD=1 cargo check
```

注意：跳过前端构建只适合开发检查；正式发布请不要设置 `SKIP_FRONTEND_BUILD=1`。

## 启动

开发模式运行后端：

```bash
SKIP_FRONTEND_BUILD=1 cargo run
```

运行 release 二进制：

```bash
./target/release/algo-station
```

默认监听 `0.0.0.0:8928`，浏览器访问：

```text
http://127.0.0.1:8928
```

前端开发服务器：

```bash
cd frontend
bun install
bun run dev
```

前端开发模式默认使用 `BACKEND_URL=http://127.0.0.1:8928` 代理 API。

## 部署

最小离线部署目录建议如下：

```text
deploy/
├── algo-station
└── data/
    ├── algo.db
    └── leetcode/
```

`algo-station` 二进制内已包含前端静态资源和原创题解；`data/leetcode` 用于读取题面、解法和参考代码；`data/algo.db` 保存题库索引和文章索引。

如果不携带 `algo.db`，服务会在数据库为空时尝试从 `data/leetcode` 自动建索引。

## 环境变量

| 变量 | 默认值 | 说明 |
| --- | --- | --- |
| `PORT` | `8928` | HTTP 服务监听端口 |
| `DATABASE_URL` | 优先 `./data/algo.db`，否则二进制同级 `data/algo.db` | SQLite 连接串，例如 `sqlite://data/algo.db?mode=rwc` |
| `LEETCODE_REPO_DIR` | 优先二进制同级 `data/leetcode`，否则 `./data/leetcode` | 本地 doocs/leetcode 路径 |
| `BACKEND_URL` | `http://127.0.0.1:8928` | 前端开发服务器代理目标 |
| `RUST_LOG` | `info` | 日志过滤，例如 `info,algo_station=debug` |
| `SKIP_FRONTEND_BUILD` | 未设置 | 设为 `1` 时跳过 `build.rs` 的前端构建 |

## 常用命令

```bash
# 后端检查
SKIP_FRONTEND_BUILD=1 cargo check

# 前端检查
cd frontend && bun run check

# 发布构建
cargo build --release

# 重建题库索引
SKIP_FRONTEND_BUILD=1 cargo run --release --bin scrape -- --repo data/leetcode

# 使用 Justfile
just backend
just frontend
just check
just check-fe
just build
```

## 添加原创题解

在 `content/articles/` 下新增 Markdown 文件，并写入 frontmatter：

```yaml
---
title: 题解标题
category: 数组与字符串
summary: 一句话说明这篇题解的训练重点
problem_ids: [1, 167]
order: 3
---
```

正文使用普通 Markdown。发布构建时，原创题解会被嵌入二进制；开发期修改文章后重启后端即可重新扫描索引。

## API

```text
GET /api/health
GET /api/stats
GET /api/tags
GET /api/problems?q=&difficulty=&tag=&has_article=&page=&page_size=
GET /api/problems/:id
GET /api/problems/:id/statement
GET /api/problems/:id/solutions
GET /api/problems/:id/solutions/:lang
GET /api/articles
GET /api/articles/:slug
```

## License

- 后端和前端代码：MIT
- 站内原创题解：见 LICENSE
- doocs/leetcode 来源内容：CC-BY-SA 4.0

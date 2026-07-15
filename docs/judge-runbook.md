# Judge Worker 运维手册

## 启动前检查

1. 主站与 Worker 的 `DATABASE_URL` 完全相同。
2. `docker info --format '{{json .SecurityOptions}}'` 包含 `rootless`。
3. `docker image inspect algo-station-go-runner:1.0` 成功。
4. `docker image inspect algo-station-rust-runner:1.0` 成功。
5. 数据库所在目录对当前用户可写，磁盘空间充足。

构建镜像：

```bash
docker build -t algo-station-go-runner:1.0 runner/go
docker build -t algo-station-rust-runner:1.0 runner/rust
```

启动顺序没有硬要求。建议先启动主站，再启动 Worker：

```bash
SKIP_FRONTEND_BUILD=1 cargo run --bin algo-station
SKIP_FRONTEND_BUILD=1 cargo run --bin judge_worker
```

检查状态：

```bash
curl -s http://127.0.0.1:8928/api/judge/status
```

`online=true` 表示最近 30 秒收到过 Worker 心跳。

## 常见故障

### Worker 启动时报 Docker permission denied

当前用户没有连接目标 daemon。不要直接把用户加入 rootful `/var/run/docker.sock` 来绕过默认安全模型；配置 rootless Docker，并确认 `DOCKER_HOST` 指向用户级 socket。

### 提交一直 queued

- 检查 `/api/judge/status`；
- 检查 Worker 日志是否成功注册；
- 确认主站和 Worker 没有使用两个不同的相对数据库路径；
- 检查 runner 镜像名称是否与环境变量一致。

### 提交一直 running

Worker 正常时每 10 秒续租。Worker 崩溃后，租约 45 秒过期，其他 Worker 会重试。连续 3 次租约过期后任务会变为 `internal_error`。

可以只读检查队列：

```sql
SELECT id, status, attempts, lease_owner, lease_until, queued_at
FROM submissions
WHERE status IN ('queued', 'running')
ORDER BY id;
```

不要在仍有健康 Worker 时手工改租约。

### 大量 compile_error

- 检查提交选择的语言和契约是否正确；
- 函数模式必须实现 Go `Solve(input string) string` 或 Rust `solve(&str) -> String`；
- runner 只提供标准库；
- 确认镜像编译器版本与 `runner/*/Dockerfile` 一致。

编译容器固定最多使用 512 MB 内存，运行容器仍服从题目限制。若日志包含 `signal: killed` 或 `no space left on device`，还应核对实际启动参数是否包含可写的 `/tmp` tmpfs（`mode=1777`），并确认 Worker 与代码版本一致。

### queue is full

API 在 `queued + running >= 20` 时返回 429。先恢复 Worker；不要简单提高上限，因为它同时是磁盘、CPU 和滥用保护。

### 容器残留

超时路径会执行 `docker rm -f`。Worker 或 Docker daemon 突然崩溃时可能残留名称以 `algo-station-` 开头的容器。确认对应任务租约已过期后再清理：

```bash
docker ps -a --filter name=algo-station-
```

## 升级 runner

1. 修改固定基础镜像版本；
2. 使用新标签构建，例如 `algo-station-rust-runner:1.1`；
3. 先用测试提交验证；
4. 设置 `JUDGE_RUST_IMAGE` / `JUDGE_GO_IMAGE` 并重启 Worker；
5. 保留旧镜像直到队列清空并确认无回滚需求。

不要在同一个标签下静默替换生产镜像，否则结果难以复现。

## 停机

发送 SIGINT。Worker 会在当前循环结束后把自己的状态改为 `offline`。若进程被强杀，心跳在 30 秒后显示离线，正在运行的任务在租约过期后重试。

# 升级、备份与回滚

## 升级前备份

最稳妥的方式是停止主站和 Worker，再复制 `data/algo.db`。如果数据库启用了 WAL，同时复制文件容易得到不一致快照；在线备份应使用 SQLite `.backup` 或 backup API。

同时保留：

- 当前 `algo-station` 和 `judge_worker` 二进制；
- 当前 runner 镜像标签；
- 当前 `content/curriculum/catalog.json` 对应的源码版本；
- 数据库备份。

## 数据库迁移

迁移是只向前执行的。新增的课程与判题表不会覆盖旧版 `training_records`、`practice_drafts` 或 `bookmarks`。旧进度迁移使用 `INSERT OR IGNORE`，重复启动不会重复覆盖新版记录。

升级后先验证：

```bash
curl -s http://127.0.0.1:8928/api/health
curl -s http://127.0.0.1:8928/api/curriculum
curl -s http://127.0.0.1:8928/api/judge/status
```

## 应用回滚

如果只回滚主站二进制，先停止 Worker。旧版程序会忽略新增表，但不了解新版课程进度。保留数据库备份，避免用户继续在两个不兼容版本间写入。

完整回滚步骤：

1. 停止主站和所有 Worker；
2. 保存故障数据库副本用于排查；
3. 恢复升级前数据库备份；
4. 恢复对应主站、Worker 和 runner 镜像；
5. 先启动主站验证只读 API，再启动 Worker；
6. 提交一条测试任务，确认队列、租约和结果回写。

不要通过删除 SQLx 迁移记录来“降级”；这会破坏迁移校验。

## 仅恢复卡住队列

优先重启 Worker，租约会自动恢复。只有在确认没有 Worker 仍执行任务时，才可把未耗尽重试的过期任务重置为 queued：

```sql
UPDATE submissions
SET status = 'queued', lease_owner = NULL, lease_until = NULL, updated_at = datetime('now')
WHERE status = 'running' AND lease_until < datetime('now') AND attempts < 3;
```

先备份数据库，并记录受影响的提交 ID。

## 课程内容回滚

课程目录编译在二进制中。回滚课程应恢复生成器、文章 frontmatter、前端 25 套动画和生成后的目录到同一个提交，然后重新运行契约测试并构建。不要只替换 JSON，否则前端可能缺少对应动态图。


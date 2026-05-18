#!/usr/bin/env bash
# 瘦身 data/leetcode：只保留 solution/ 下题目目录里的 README.md / README_EN.md。
#
# 用法：
#   scripts/cleanup_leetcode.sh --dry-run   # 打印将要删除的内容
#   scripts/cleanup_leetcode.sh             # 实际执行删除
#
# 删除策略：
#   1) 整个 data/leetcode/.git/ 目录
#   2) data/leetcode/ 下除 solution/ 之外的所有顶层目录和文件
#   3) data/leetcode/solution/ 下每个题目目录里，除 README.md / README_EN.md 之外的所有文件
#      （保留分组索引 solution/<bucket>/README.md 和总索引 solution/README.md）

set -euo pipefail

DRY_RUN=0
if [[ "${1:-}" == "--dry-run" ]]; then
  DRY_RUN=1
fi

# 解析仓库根（脚本所在目录的上一级）
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
TARGET="${ROOT}/data/leetcode"

if [[ ! -d "${TARGET}/solution" ]]; then
  echo "错误：${TARGET} 不存在或不包含 solution/，请先 clone doocs/leetcode 到 data/leetcode" >&2
  exit 1
fi

run() {
  if [[ "${DRY_RUN}" -eq 1 ]]; then
    echo "[DRY-RUN] $*"
  else
    echo "[执行]   $*"
    eval "$@"
  fi
}

echo "→ 目标目录：${TARGET}"
echo "→ 模式：$([[ ${DRY_RUN} -eq 1 ]] && echo dry-run || echo 实际执行)"
echo

# 1) 删除 .git/（占大头 ~780MB）
if [[ -d "${TARGET}/.git" ]]; then
  run "rm -rf '${TARGET}/.git'"
fi

# 2) 删除非 solution 的顶层文件和目录
echo
echo "[阶段 2] 清理顶层非 solution 内容"
while IFS= read -r -d '' entry; do
  name="$(basename "${entry}")"
  if [[ "${name}" == "solution" ]]; then
    continue
  fi
  run "rm -rf '${entry}'"
done < <(find "${TARGET}" -mindepth 1 -maxdepth 1 -print0)

# 3) 清理 solution/ 顶层除 README*.md 之外的辅助文件（rating/contest/template 等）
echo
echo "[阶段 3] 清理 solution/ 顶层非题面辅助文件"
while IFS= read -r -d '' file; do
  base="$(basename "${file}")"
  if [[ "${base}" == "README.md" || "${base}" == "README_EN.md" ]]; then
    continue
  fi
  run "rm -f '${file}'"
done < <(find "${TARGET}/solution" -mindepth 1 -maxdepth 1 -type f -print0)

# 4) 清理每个题目目录里除 README.md / README_EN.md 之外的文件
#    题目目录形如：solution/<bucket>/<NNNN.Title>/
#    分组索引 solution/<bucket>/README.md 和总索引 solution/README.md 都保留。
echo
echo "[阶段 4] 清理题目目录里的非题面文件"
deleted_count=0
while IFS= read -r -d '' file; do
  base="$(basename "${file}")"
  if [[ "${base}" == "README.md" || "${base}" == "README_EN.md" ]]; then
    continue
  fi
  if [[ "${DRY_RUN}" -eq 1 ]]; then
    deleted_count=$((deleted_count + 1))
    if [[ "${deleted_count}" -le 20 ]]; then
      echo "[DRY-RUN] rm '${file}'"
    fi
  else
    rm -f "${file}"
    deleted_count=$((deleted_count + 1))
  fi
done < <(find "${TARGET}/solution" -mindepth 3 -type f -print0)

if [[ "${DRY_RUN}" -eq 1 && "${deleted_count}" -gt 20 ]]; then
  echo "[DRY-RUN] ... 共 ${deleted_count} 个文件将被删除（仅列前 20）"
elif [[ "${DRY_RUN}" -eq 0 ]]; then
  echo "[执行]   阶段 4 删除 ${deleted_count} 个非题面文件"
fi

# 4) 清理留下的空目录（深度优先）
if [[ "${DRY_RUN}" -eq 0 ]]; then
  find "${TARGET}/solution" -mindepth 2 -type d -empty -delete 2>/dev/null || true
fi

echo
if [[ "${DRY_RUN}" -eq 1 ]]; then
  echo "→ dry-run 完成。无修改。再次以 '不带 --dry-run' 执行真正删除。"
else
  echo "→ 完成。当前体积："
  du -sh "${TARGET}"
  echo "→ 剩余非 README 文件（应为空）："
  find "${TARGET}" -type f ! -name 'README.md' ! -name 'README_EN.md' | head -5 || true
fi

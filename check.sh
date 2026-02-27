#!/bin/bash
# OSCamp 练习检查脚本
# 逐个检查每个练习的测试状态。第四章（riscv64）在 x86 上通过 QEMU 用户态运行，无需单独脚本。

set -e

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

PASS=0
FAIL=0
SKIP=0

# 仓库根目录（本脚本所在目录）
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# 第四章使用的 target 与 runner（绝对路径，保证任意目录执行 check.sh 都能跑）
RISCV64_TARGET="riscv64gc-unknown-linux-gnu"
RISCV64_SYSROOT="${RISCV64_SYSROOT:-/usr/riscv64-linux-gnu}"

# 确保 riscv64 工具链与运行环境就绪（仅在需要跑第四章且本机为 Linux 非 riscv64 时执行）
# macOS 不执行 setup，由下方逻辑对第四章做 SKIP。
ensure_riscv64_ready() {
    local arch
    arch=$(uname -m)
    if [ "$arch" = "riscv64" ]; then
        return 0
    fi
    if [ "$(uname -s)" = "Darwin" ]; then
        return 0
    fi

    if [ -d "${REPO_ROOT}/scripts" ]; then
        chmod +x "${REPO_ROOT}/scripts/setup_riscv64.sh" "${REPO_ROOT}/.cargo/run_riscv64.sh" 2>/dev/null || true
        echo -e "  ${YELLOW}[第四章] 正在准备 riscv64 环境（target / QEMU / sysroot）...${NC}"
        (cd "$REPO_ROOT" && bash scripts/setup_riscv64.sh) || exit 1
    fi
    export CARGO_TARGET_RISCV64GC_UNKNOWN_LINUX_GNU_RUNNER="bash ${REPO_ROOT}/.cargo/run_riscv64.sh"

    if ! command -v qemu-riscv64 >/dev/null 2>&1; then
        echo -e "${RED}错误: 未找到 qemu-riscv64，无法在非 riscv64 主机上运行第四章测试。${NC}" >&2
        echo "请安装 QEMU 用户态，例如：" >&2
        echo "  Debian/Ubuntu: sudo apt-get install qemu-user-static" >&2
        echo "  Fedora:        sudo dnf install qemu-user-static" >&2
        exit 1
    fi

    if [ ! -d "$RISCV64_SYSROOT" ]; then
        echo -e "${RED}错误: 未找到 riscv64 根文件系统: ${RISCV64_SYSROOT}${NC}" >&2
        echo "请安装 riscv64 交叉工具链/库，例如：" >&2
        echo "  Debian/Ubuntu: sudo apt-get install gcc-riscv64-linux-gnu" >&2
        echo "或设置: export RISCV64_SYSROOT=/path/to/riscv64/sysroot" >&2
        exit 1
    fi
}

exercises=(
    "01_concurrency_sync:thread_spawn:线程创建"
    "01_concurrency_sync:mutex_counter:Mutex共享状态"
    "01_concurrency_sync:channel:Channel通道"
    "01_concurrency_sync:process_pipe:进程管道"
    "02_no_std_dev:global_allocator_without_free:全局分配器(无释放)"
    "02_no_std_dev:allocator_with_free:带释放的分配器"
    "02_no_std_dev:raw_syscall:原始系统调用"
    "02_no_std_dev:file_descriptor:文件描述符"
    "03_os_concurrency:atomic_counter:原子计数器"
    "03_os_concurrency:atomic_ordering:内存序"
    "03_os_concurrency:spinlock:自旋锁"
    "03_os_concurrency:spinlock_guard:RAII自旋锁守卫"
    "03_os_concurrency:rwlock:读写锁(写者优先)"
    "04_context_switch:stack_coroutine:有栈协程"
    "04_context_switch:green_threads:绿色线程"
    "05_async_programming:basic_future:手动实现Future"
    "05_async_programming:tokio_tasks:Tokio异步任务"
    "05_async_programming:async_channel_ex:异步通道"
    "05_async_programming:select_timeout:Select与超时"
)

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}   OS Camp - Rust & OS 进阶实验检查${NC}"
echo -e "${BLUE}========================================${NC}"
echo ""

current_module=""
riscv64_ready=0

for entry in "${exercises[@]}"; do
    IFS=':' read -r module package desc <<< "$entry"

    if [ "$module" != "$current_module" ]; then
        current_module="$module"
        echo -e "\n${YELLOW}[$module]${NC}"
        # 进入第四章前确保 riscv64 环境就绪
        if [ "$module" = "04_context_switch" ] && [ "$riscv64_ready" -eq 0 ]; then
            ensure_riscv64_ready
            riscv64_ready=1
        fi
    fi

    printf "  %-25s %-20s " "$desc" "($package)"

    if [ "$module" = "04_context_switch" ]; then
        if [ "$(uname -s)" = "Darwin" ]; then
            echo -e "${YELLOW}SKIP (macOS 请用 Linux 或 cross)${NC}"
            ((SKIP++))
        elif cargo test -p "$package" --target "$RISCV64_TARGET" --quiet -- --nocapture 2>/dev/null; then
            echo -e "${GREEN}PASS${NC}"
            ((PASS++))
        else
            echo -e "${RED}FAIL${NC}"
            ((FAIL++))
        fi
    else
        if cargo test -p "$package" --quiet 2>/dev/null; then
            echo -e "${GREEN}PASS${NC}"
            ((PASS++))
        else
            echo -e "${RED}FAIL${NC}"
            ((FAIL++))
        fi
    fi
done

echo ""
echo -e "${BLUE}========================================${NC}"
TOTAL=$((PASS + FAIL + SKIP))
echo -e "  总计: ${GREEN}$PASS${NC} 通过 / ${RED}$FAIL${NC} 未通过 / ${YELLOW}$SKIP${NC} 跳过 / $TOTAL 总题"
echo -e "  进度: $PASS/$TOTAL"
echo -e "${BLUE}========================================${NC}"

if [ $FAIL -eq 0 ]; then
    echo -e "\n${GREEN}恭喜！所有练习全部通过！${NC}"
    exit 0
else
    echo -e "\n${YELLOW}还有 $FAIL 个练习需要完成，加油！${NC}"
    exit 1
fi

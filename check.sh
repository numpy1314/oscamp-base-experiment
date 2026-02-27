#!/bin/bash
# OSCamp 练习检查脚本
# 逐个检查每个练习的测试状态

set -e

GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

PASS=0
FAIL=0
SKIP=0

exercises=(
    "01_concurrency_sync:thread_spawn:线程创建"
    "01_concurrency_sync:mutex_counter:Mutex共享状态"
    "01_concurrency_sync:channel:Channel通道"
    "01_concurrency_sync:process_pipe:进程管道"
    "02_no_std_dev:global_allocator:全局分配器"
    "02_no_std_dev:raw_syscall:原始系统调用"
    "02_no_std_dev:file_descriptor:文件描述符"
    "03_os_concurrency:atomic_counter:原子计数器"
    "03_os_concurrency:atomic_ordering:内存序"
    "03_os_concurrency:spinlock:自旋锁"
    "03_os_concurrency:spinlock_guard:RAII自旋锁守卫"
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

for entry in "${exercises[@]}"; do
    IFS=':' read -r module package desc <<< "$entry"

    if [ "$module" != "$current_module" ]; then
        current_module="$module"
        echo -e "\n${YELLOW}[$module]${NC}"
    fi

    printf "  %-25s %-20s " "$desc" "($package)"

    if cargo test -p "$package" --quiet 2>/dev/null; then
        echo -e "${GREEN}PASS${NC}"
        ((PASS++))
    else
        echo -e "${RED}FAIL${NC}"
        ((FAIL++))
    fi
done

echo ""
echo -e "${BLUE}========================================${NC}"
TOTAL=$((PASS + FAIL))
echo -e "  总计: ${GREEN}$PASS${NC} 通过 / ${RED}$FAIL${NC} 未通过 / $TOTAL 总题"
echo -e "  进度: $PASS/$TOTAL"
echo -e "${BLUE}========================================${NC}"

if [ $FAIL -eq 0 ]; then
    echo -e "\n${GREEN}恭喜！所有练习全部通过！${NC}"
    exit 0
else
    echo -e "\n${YELLOW}还有 $FAIL 个练习需要完成，加油！${NC}"
    exit 1
fi

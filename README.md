# OS Camp - Rust & OS 进阶实验

一个仿 [rustlings](https://github.com/rust-lang/rustlings) 风格的 Rust 进阶与操作系统入门练习仓库。
通过补全代码、通过测试的方式，学习 Rust 并发编程、异步编程、`no_std` 开发以及操作系统核心概念。

## 前置要求

- Rust toolchain (stable, >= 1.75)
- Linux x86_64 环境（部分练习涉及 syscall 和内联汇编）

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 练习结构

共 **6 大模块、21 个练习**，由浅入深：

### 模块一：并发（同步）— `01_concurrency_sync/`

| # | 练习 | 知识点 |
|---|------|--------|
| 1 | `01_thread_spawn` | `thread::spawn`, `move` 闭包, `join` |
| 2 | `02_mutex_counter` | `Arc<Mutex<T>>`, 共享状态并发 |
| 3 | `03_channel` | `mpsc::channel`, 多生产者模式 |
| 4 | `04_process_pipe` | `Command`, `Stdio::piped()`, 进程管道 |

### 模块二：异步编程 — `02_async_programming/`

| # | 练习 | 知识点 |
|---|------|--------|
| 1 | `01_basic_future` | 手动实现 `Future` trait, `Poll`, `Waker` |
| 2 | `02_tokio_tasks` | `tokio::spawn`, `JoinHandle`, 并发任务 |
| 3 | `03_async_channel` | `tokio::sync::mpsc`, 异步生产者-消费者 |
| 4 | `04_select_timeout` | `tokio::select!`, 超时控制, 竞态执行 |

### 模块三：no_std 开发 — `03_no_std_dev/`

| # | 练习 | 知识点 |
|---|------|--------|
| 1 | `01_global_allocator` | `GlobalAlloc` trait, Bump 分配器, 内存对齐 |
| 2 | `02_raw_syscall` | `asm!` 内联汇编, Linux syscall 调用约定 |
| 3 | `03_file_descriptor` | 文件描述符, RAII, open/read/write/close |

### 模块四：OS 并发进阶 — `04_os_concurrency/`

| # | 练习 | 知识点 |
|---|------|--------|
| 1 | `01_atomic_counter` | `AtomicU64`, `fetch_add`, CAS 循环 |
| 2 | `02_atomic_ordering` | 内存序, Release-Acquire, `OnceCell` |
| 3 | `03_spinlock` | 自旋锁实现, `compare_exchange`, `spin_loop` |
| 4 | `04_spinlock_guard` | RAII 守卫, `Deref`/`DerefMut`/`Drop` |

### 模块五：上下文切换 — `05_context_switch/`

| # | 练习 | 知识点 |
|---|------|--------|
| 1 | `01_stack_coroutine` | callee-saved 寄存器, 栈帧, 上下文切换 |
| 2 | `02_green_threads` | 绿色线程调度器, 协作式调度, yield |

### 模块六：页表 — `06_page_table/`

| # | 练习 | 知识点 |
|---|------|--------|
| 1 | `01_pte_flags` | SV39 PTE 位布局, 位运算构造/解析页表项 |
| 2 | `02_page_table_walk` | 单级页表, VPN/offset 拆分, 地址翻译, 缺页 |
| 3 | `03_multi_level_pt` | SV39 三级页表, 页表遍历, 大页(2MB)映射 |
| 4 | `04_tlb_sim` | TLB 查找/插入/FIFO替换, 刷新(全部/按页/按ASID), MMU 模拟 |

## 快速开始

```bash
# 1. 克隆仓库
git clone <repo-url> && cd oscamp-base-experiment

# 2. 构建交互式 CLI 工具
cargo build -p oscamp-cli

# 3. 启动交互式练习模式（推荐）
./target/debug/oscamp watch
```

## 交互式 CLI 工具 (`oscamp`)

内置了类似 rustlings 的交互式终端工具，支持实时文件监听和进度追踪：

```bash
oscamp              # 启动交互式 watch 模式（默认）
oscamp watch        # 同上
oscamp list         # 查看所有练习完成状态
oscamp check        # 批量检查所有练习
oscamp run <包名>   # 运行指定练习测试
oscamp hint <包名>  # 查看练习提示
oscamp help         # 显示帮助
```

### Watch 模式功能

- **自动检测文件变化**：保存文件后自动重新运行测试
- **自动跳转**：当前练习通过后自动跳到下一道未完成的题目
- **实时进度条**：显示整体完成进度
- **快捷键**：
  - `h` — 查看当前练习的提示
  - `l` — 查看所有练习列表
  - `n` / `p` — 下一题 / 上一题
  - `r` / `Enter` — 重新运行测试
  - `q` / `Esc` — 退出

### 也可以手动运行

```bash
# 运行某个练习的测试
cargo test -p thread_spawn

# 查看详细输出
cargo test -p thread_spawn -- --nocapture

# 检查所有练习
cargo test --workspace
```

## 答题流程

1. **启动**：运行 `./target/debug/oscamp watch` 进入交互模式
2. **阅读**：打开当前练习文件 `src/lib.rs`，阅读顶部文档了解知识点
3. **编码**：找到 `todo!()` 标记，根据注释提示补全代码
4. **保存**：保存文件后，CLI 自动重新测试
5. **通过**：测试通过后自动跳到下一题，按 `h` 可随时查看提示

## 注意事项

- 模块三和模块五的部分练习需要 **Linux x86_64** 环境
- 带 `#[cfg(target_os = "linux")]` 或 `#[cfg(target_arch = "x86_64")]` 的测试在其他平台会被跳过
- 建议按模块顺序进行，每个模块内的练习也是递进式的

## License

MIT

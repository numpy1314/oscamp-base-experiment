# OS Camp - Rust & OS Advanced Experiments

A Rust advanced and operating system introductory exercise repository in the style of [rustlings](https://github.com/rust-lang/rustlings).
Learn Rust concurrency programming, async programming, `no_std` development, and operating system core concepts through completing code and passing tests.

## Prerequisites

- Rust toolchain (stable, >= 1.75)
- Linux 环境：多数练习为 x86_64；**第四章（上下文切换）仅支持 riscv64**，需在 riscv64 环境或使用 QEMU 用户态运行

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Exercise Structure

**6 modules, 23 exercises** in total, from easy to advanced:

### Module 1: Concurrency (Synchronous) — `01_concurrency_sync/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_thread_spawn` | `thread::spawn`, `move` closures, `join` |
| 2 | `02_mutex_counter` | `Arc<Mutex<T>>`, shared state concurrency |
| 3 | `03_channel` | `mpsc::channel`, multiple producer pattern |
| 4 | `04_process_pipe` | `Command`, `Stdio::piped()`, process pipes |

### Module 2: no_std Development — `02_no_std_dev/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_global_allocator_without_free` | `GlobalAlloc` trait, Bump allocator (no dealloc), memory alignment |
| 2 | `02_allocator_with_free` | Free-list allocator, alloc + dealloc, block reuse |
| 3 | `03_raw_syscall` | `asm!` inline assembly, Linux syscall calling convention |
| 4 | `04_file_descriptor` | File descriptors, RAII, open/read/write/close |

### Module 3: OS Concurrency Advanced — `03_os_concurrency/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_atomic_counter` | `AtomicU64`, `fetch_add`, CAS loop |
| 2 | `02_atomic_ordering` | Memory ordering, Release-Acquire, `OnceCell` |
| 3 | `03_spinlock` | Spinlock implementation, `compare_exchange`, `spin_loop` |
| 4 | `04_spinlock_guard` | RAII guard, `Deref`/`DerefMut`/`Drop` |
| 5 | `05_rwlock` | Writer-priority read-write lock from scratch (no `std::sync::RwLock`) |

### Module 4: Context Switching — `04_context_switch/`（仅 riscv64）

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_stack_coroutine` | Callee-saved registers, stack frames, context switching |
| 2 | `02_green_threads` | Green thread scheduler, cooperative scheduling, yield |

第四章仅在 **riscv64** 下运行；与整仓一致：直接执行 `./check.sh` 或使用 `oscamp` 即可，无需单独脚本。详见 `exercises/04_context_switch/README.md`。

### Module 5: Async Programming — `05_async_programming/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_basic_future` | Manual implementation of `Future` trait, `Poll`, `Waker` |
| 2 | `02_tokio_tasks` | `tokio::spawn`, `JoinHandle`, concurrent tasks |
| 3 | `03_async_channel` | `tokio::sync::mpsc`, async producer-consumer |
| 4 | `04_select_timeout` | `tokio::select!`, timeout control, race execution |

### Module 6: Page Tables — `06_page_table/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_pte_flags` | SV39 PTE bit layout, bit operations to construct/parse page table entries |
| 2 | `02_page_table_walk` | Single-level page tables, VPN/offset splitting, address translation, page faults |
| 3 | `03_multi_level_pt` | SV39 three-level page tables, page table walk, huge pages (2MB) mapping |
| 4 | `04_tlb_sim` | TLB lookup/insert/FIFO replacement, flush (all/by page/by ASID), MMU simulation |

## Quick Start

```bash
# 1. Clone repository
git clone <repo-url> && cd oscamp-base-experiment

# 2. Build interactive CLI tool
cargo build -p oscamp-cli

# 3. Start interactive exercise mode (recommended)
./target/debug/oscamp watch
```

## Interactive CLI Tool (`oscamp`)

Built-in interactive terminal tool similar to rustlings, supporting real-time file watching and progress tracking:

```bash
oscamp              # Start interactive watch mode (default)
oscamp watch        # Same as above
oscamp list         # View completion status of all exercises
oscamp check        # Check all exercises in batch
oscamp run <pkg>    # Run tests for specified exercise
oscamp hint <pkg>   # View exercise hint
oscamp help         # Show help
```

### Watch Mode Features

- **Automatic file change detection**: Automatically re-run tests after saving files
- **Auto-jump**: Automatically jump to next unfinished exercise after current one passes
- **Real-time progress bar**: Show overall completion progress
- **Shortcuts**:
  - `h` — View hint for current exercise
  - `l` — View list of all exercises
  - `n` / `p` — Next / Previous exercise
  - `r` / `Enter` — Re-run tests
  - `q` / `Esc` — Quit

### Manual Execution

```bash
# Run tests for a specific exercise
cargo test -p thread_spawn

# View detailed output
cargo test -p thread_spawn -- --nocapture

# Check all exercises
cargo test --workspace
```

## Workflow

1. **Start**: Run `./target/debug/oscamp watch` to enter interactive mode
2. **Read**: Open current exercise file `src/lib.rs`, read documentation to understand concepts
3. **Code**: Find `todo!()` markers, complete code according to comment hints
4. **Save**: After saving file, CLI automatically re-runs tests
5. **Pass**: After passing tests, automatically jump to next exercise; press `h` to view hints anytime

## Notes

- 部分练习（如 Module 2 系统调用、Module 4 汇编）需 **Linux** 环境；Module 4 仅支持 **riscv64**
- 建议按模块顺序完成；同一模块内题目由浅入深

## License

MIT

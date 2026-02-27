# OS Camp - Rust & OS Advanced Experiments

A Rust advanced and operating system introductory exercise repository in the style of [rustlings](https://github.com/rust-lang/rustlings).
Learn Rust concurrency programming, async programming, `no_std` development, and operating system core concepts through completing code and passing tests.

## Prerequisites

- Rust toolchain (stable, >= 1.75)
- Linux x86_64 environment (some exercises involve syscall and inline assembly)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Exercise Structure

**6 modules, 21 exercises** in total, from easy to advanced:

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
| 1 | `01_global_allocator` | `GlobalAlloc` trait, Bump allocator, memory alignment |
| 2 | `02_raw_syscall` | `asm!` inline assembly, Linux syscall calling convention |
| 3 | `03_file_descriptor` | File descriptors, RAII, open/read/write/close |

### Module 3: OS Concurrency Advanced — `03_os_concurrency/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_atomic_counter` | `AtomicU64`, `fetch_add`, CAS loop |
| 2 | `02_atomic_ordering` | Memory ordering, Release-Acquire, `OnceCell` |
| 3 | `03_spinlock` | Spinlock implementation, `compare_exchange`, `spin_loop` |
| 4 | `04_spinlock_guard` | RAII guard, `Deref`/`DerefMut`/`Drop` |

### Module 4: Context Switching — `04_context_switch/`

| # | Exercise | Concepts |
|---|----------|----------|
| 1 | `01_stack_coroutine` | Callee-saved registers, stack frames, context switching |
| 2 | `02_green_threads` | Green thread scheduler, cooperative scheduling, yield |

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

- Some exercises in Module 2 and Module 4 require **Linux x86_64** environment
- Tests with `#[cfg(target_os = "linux")]` or `#[cfg(target_arch = "x86_64")]` are skipped on other platforms
- Recommended to follow the module order shown above; exercises within each module are progressive

## License

MIT

//! # 绿色线程调度器
//!
//! 本练习中，你需要基于上下文切换实现一个简单的绿色线程（协作式）调度器。
//!
//! ## 知识点
//! - 协作式调度 vs 抢占式调度
//! - 任务状态机：Ready, Running, Finished
//! - yield 主动让出 CPU
//! - 调度器的运行循环
//!
//! ## 设计
//! 每个绿色线程有自己的栈和上下文。
//! 线程通过调用 `yield_now()` 主动让出执行权。
//! 调度器轮转选择下一个就绪的线程执行。

use std::arch::asm;

const STACK_SIZE: usize = 1024 * 64;
const MAX_THREADS: usize = 8;

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct TaskContext {
    rsp: u64,
    rbx: u64,
    rbp: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThreadState {
    Ready,
    Running,
    Finished,
}

struct GreenThread {
    ctx: TaskContext,
    state: ThreadState,
    _stack: Option<Vec<u8>>,
}

/// 绿色线程调度器
pub struct Scheduler {
    threads: Vec<GreenThread>,
    current: usize,
}

static mut SCHEDULER: *mut Scheduler = std::ptr::null_mut();

#[cfg(target_arch = "x86_64")]
unsafe fn switch_context(old: &mut TaskContext, new: &TaskContext) {
    asm!(
        "mov [rdi + 0x00], rsp",
        "mov [rdi + 0x08], rbx",
        "mov [rdi + 0x10], rbp",
        "mov [rdi + 0x18], r12",
        "mov [rdi + 0x20], r13",
        "mov [rdi + 0x28], r14",
        "mov [rdi + 0x30], r15",
        "mov rsp, [rsi + 0x00]",
        "mov rbx, [rsi + 0x08]",
        "mov rbp, [rsi + 0x10]",
        "mov r12, [rsi + 0x18]",
        "mov r13, [rsi + 0x20]",
        "mov r14, [rsi + 0x28]",
        "mov r15, [rsi + 0x30]",
        "ret",
        in("rdi") old as *mut TaskContext as u64,
        in("rsi") new as *const TaskContext as u64,
        clobber_abi("C"),
    );
}

impl Scheduler {
    pub fn new() -> Self {
        let main_thread = GreenThread {
            ctx: TaskContext::default(),
            state: ThreadState::Running,
            _stack: None, // 主线程使用系统栈
        };

        Self {
            threads: vec![main_thread],
            current: 0,
        }
    }

    /// 注册一个新的绿色线程。
    ///
    /// TODO:
    /// 1. 分配 STACK_SIZE 字节的栈
    /// 2. 计算栈顶地址
    /// 3. 初始化 TaskContext（在栈顶放置 thread_wrapper 的地址作为入口）
    /// 4. 将新线程加入 threads 列表，状态设为 Ready
    ///
    /// 注意：不能直接用 `f` 作为入口点，因为 `f` 执行完后需要标记线程为 Finished。
    /// 使用 `thread_wrapper` 作为入口，将 `f` 存储在某处供 wrapper 调用。
    ///
    /// 简化方案：entry 函数签名为 extern "C" fn()，直接使用它作为入口。
    pub fn spawn(&mut self, entry: extern "C" fn()) {
        // TODO: 分配栈
        // TODO: 初始化上下文（栈顶放入 guard 函数地址，然后是 entry 地址）
        // TODO: 创建 GreenThread 并推入 self.threads
        todo!()
    }

    /// 运行调度器直到所有线程完成。
    ///
    /// TODO: 循环调用 schedule_next()，直到只剩主线程（其他线程都 Finished）。
    pub fn run(&mut self) {
        // TODO: 将 SCHEDULER 设为 self
        // TODO: 循环检查是否有非 Finished 的非主线程
        // TODO: 如果有，调用 yield_now() 切换
        // TODO: 全部完成后清理
        todo!()
    }

    /// 找到下一个 Ready 的线程并切换过去。
    ///
    /// TODO:
    /// 1. 从 current+1 开始轮询，找到第一个 Ready 的线程
    /// 2. 将当前线程状态从 Running 改为 Ready（如果不是 Finished）
    /// 3. 将目标线程状态改为 Running
    /// 4. 调用 switch_context 切换
    fn schedule_next(&mut self) {
        // TODO
        todo!()
    }
}

/// 当前运行的绿色线程主动让出 CPU。
/// 调度器将选择下一个就绪线程运行。
pub fn yield_now() {
    unsafe {
        if !SCHEDULER.is_null() {
            (*SCHEDULER).schedule_next();
        }
    }
}

/// 标记当前线程为 Finished 并切换。
/// 绿色线程函数返回后会调用此函数。
fn thread_finished() {
    unsafe {
        if !SCHEDULER.is_null() {
            let sched = &mut *SCHEDULER;
            sched.threads[sched.current].state = ThreadState::Finished;
            sched.schedule_next();
        }
    }
}

#[cfg(test)]
#[cfg(target_arch = "x86_64")]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static EXEC_ORDER: AtomicU32 = AtomicU32::new(0);

    extern "C" fn task_a() {
        EXEC_ORDER.fetch_add(1, Ordering::SeqCst); // +1 = 1
        yield_now();
        EXEC_ORDER.fetch_add(10, Ordering::SeqCst); // +10 = 12
        yield_now();
        EXEC_ORDER.fetch_add(100, Ordering::SeqCst); // +100 = 112
    }

    extern "C" fn task_b() {
        EXEC_ORDER.fetch_add(1, Ordering::SeqCst); // +1 = 2
        yield_now();
        EXEC_ORDER.fetch_add(10, Ordering::SeqCst); // +10 = 22
    }

    #[test]
    fn test_scheduler_runs_all() {
        EXEC_ORDER.store(0, Ordering::SeqCst);

        let mut sched = Scheduler::new();
        sched.spawn(task_a);
        sched.spawn(task_b);
        sched.run();

        let total = EXEC_ORDER.load(Ordering::SeqCst);
        // task_a: 1 + 10 + 100 = 111
        // task_b: 1 + 10 = 11
        // total = 122
        assert_eq!(total, 122);
    }

    static SIMPLE_FLAG: AtomicU32 = AtomicU32::new(0);

    extern "C" fn simple_task() {
        SIMPLE_FLAG.store(42, Ordering::SeqCst);
    }

    #[test]
    fn test_single_thread() {
        SIMPLE_FLAG.store(0, Ordering::SeqCst);

        let mut sched = Scheduler::new();
        sched.spawn(simple_task);
        sched.run();

        assert_eq!(SIMPLE_FLAG.load(Ordering::SeqCst), 42);
    }
}

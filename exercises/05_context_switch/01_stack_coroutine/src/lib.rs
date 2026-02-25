//! # 有栈协程与上下文切换
//!
//! 本练习中，你需要使用内联汇编实现最基本的上下文切换，
//! 这是操作系统线程调度的核心机制。
//!
//! ## 知识点
//! - callee-saved 寄存器：rbx, rbp, r12, r13, r14, r15
//! - 栈指针 rsp 的保存与恢复
//! - 通过栈上的返回地址控制执行流
//! - 内联汇编 `core::arch::asm!`
//!
//! ## x86_64 调用约定
//! caller-saved: rax, rcx, rdx, rsi, rdi, r8-r11 (由调用者保存)
//! callee-saved: rbx, rbp, r12-r15, rsp (由被调用者保存)

use std::arch::asm;

/// 任务上下文：保存恢复执行所需的寄存器状态。
#[repr(C)]
#[derive(Debug, Default)]
pub struct TaskContext {
    pub rsp: u64,
    pub rbx: u64,
    pub rbp: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
}

impl TaskContext {
    pub const fn empty() -> Self {
        Self {
            rsp: 0, rbx: 0, rbp: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
        }
    }

    /// 初始化上下文，使其在切换后从 `entry` 函数开始执行。
    ///
    /// TODO:
    /// 1. 将 entry 函数地址压入栈顶（作为返回地址）
    ///    - stack_top 必须 16 字节对齐
    ///    - 在 stack_top 处放置 entry 的地址
    ///    - rsp 设为 stack_top（ret 指令会弹出该地址并跳转）
    /// 2. 其余寄存器设为 0
    ///
    /// 注意：stack_top 是栈的**高地址端**（栈向低地址增长）
    pub fn init(&mut self, stack_top: usize, entry: usize) {
        // TODO: 在栈顶放置 entry 地址（模拟 call 指令压栈的返回地址）
        // unsafe { *(stack_top as *mut usize).sub(1) = entry; }
        // self.rsp = stack_top as u64 - 8; // 指向返回地址
        todo!()
    }
}

/// 从 `old` 上下文切换到 `new` 上下文。
///
/// TODO: 使用内联汇编实现上下文切换：
/// 1. 保存当前 callee-saved 寄存器到 `old`
/// 2. 保存当前 rsp 到 `old.rsp`
/// 3. 从 `new` 恢复 rsp
/// 4. 从 `new` 恢复 callee-saved 寄存器
/// 5. ret 指令将跳转到 new 上下文的返回地址
///
/// 提示：rdi = old 指针, rsi = new 指针（System V AMD64 调用约定）
///
/// ```text
/// asm!(
///     // 保存 callee-saved 寄存器到 old
///     "mov [rdi + 0x00], rsp",
///     "mov [rdi + 0x08], rbx",
///     "mov [rdi + 0x10], rbp",
///     "mov [rdi + 0x18], r12",
///     "mov [rdi + 0x20], r13",
///     "mov [rdi + 0x28], r14",
///     "mov [rdi + 0x30], r15",
///     // 从 new 恢复
///     "mov rsp, [rsi + 0x00]",
///     "mov rbx, [rsi + 0x08]",
///     "mov rbp, [rsi + 0x10]",
///     "mov r12, [rsi + 0x18]",
///     "mov r13, [rsi + 0x20]",
///     "mov r14, [rsi + 0x28]",
///     "mov r15, [rsi + 0x30]",
///     "ret",
///     in("rdi") old as *mut TaskContext as u64,
///     in("rsi") new as *const TaskContext as u64,
///     // clobbers: 所有 caller-saved 寄存器
///     clobber_abi("C"),
/// )
/// ```
#[cfg(target_arch = "x86_64")]
pub unsafe fn switch_context(old: &mut TaskContext, new: &TaskContext) {
    // TODO: 实现上下文切换
    todo!()
}

const STACK_SIZE: usize = 1024 * 64; // 64KB per coroutine stack

/// 分配一个协程栈，返回栈顶地址（高地址端）。
///
/// TODO: 使用 Vec<u8> 分配 STACK_SIZE 字节
/// 返回缓冲区的**末尾**地址（栈从高地址向低地址增长）
pub fn alloc_stack() -> (Vec<u8>, usize) {
    // TODO: 分配 STACK_SIZE 字节的 Vec
    // 计算栈顶地址 = buffer.as_ptr() + STACK_SIZE
    // 返回 (buffer, stack_top)
    // 注意: 必须保留 buffer 的所有权，否则内存会被释放
    todo!()
}

#[cfg(test)]
#[cfg(target_arch = "x86_64")]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};

    static COUNTER: AtomicU32 = AtomicU32::new(0);

    extern "C" fn task_entry() {
        COUNTER.store(42, Ordering::SeqCst);
        // 不能从这里返回（没有有效的返回地址），所以我们循环
        loop { std::hint::spin_loop(); }
    }

    #[test]
    fn test_alloc_stack() {
        let (buf, top) = alloc_stack();
        assert_eq!(top, buf.as_ptr() as usize + STACK_SIZE);
        assert!(top % 8 == 0); // 基本对齐
    }

    #[test]
    fn test_context_init() {
        let (buf, top) = alloc_stack();
        let _ = buf; // keep alive
        let mut ctx = TaskContext::empty();
        let entry = task_entry as usize;
        ctx.init(top, entry);
        assert_eq!(ctx.rsp, (top - 8) as u64);
        // 栈顶应存放 entry 地址
        unsafe {
            let ret_addr = *((top - 8) as *const usize);
            assert_eq!(ret_addr, entry);
        }
    }

    #[test]
    fn test_switch_to_task() {
        COUNTER.store(0, Ordering::SeqCst);

        let (_stack_buf, stack_top) = alloc_stack();
        let mut main_ctx = TaskContext::empty();
        let mut task_ctx = TaskContext::empty();

        // 设置任务：切换到 task_entry 后设置 COUNTER=42
        // 但 task_entry 中会死循环，所以我们用另一种方式测试

        // 更简单的测试：创建一个任务，该任务切换回主上下文
        static mut MAIN_CTX_PTR: *mut TaskContext = std::ptr::null_mut();
        static mut TASK_CTX_PTR: *mut TaskContext = std::ptr::null_mut();

        extern "C" fn cooperative_task() {
            COUNTER.store(99, Ordering::SeqCst);
            // 切换回主上下文
            unsafe {
                switch_context(&mut *TASK_CTX_PTR, &*MAIN_CTX_PTR);
            }
        }

        task_ctx.init(stack_top, cooperative_task as usize);

        unsafe {
            MAIN_CTX_PTR = &mut main_ctx;
            TASK_CTX_PTR = &mut task_ctx;
            switch_context(&mut main_ctx, &task_ctx);
        }

        // 任务执行后，COUNTER 应为 99
        assert_eq!(COUNTER.load(Ordering::SeqCst), 99);
    }
}

//! # 原始系统调用
//!
//! 本练习中，你需要使用 `core::arch::asm!` 直接发起 Linux 系统调用，
//! 理解用户态程序与内核的交互方式。
//!
//! ## 知识点
//! - x86_64 Linux syscall 调用约定
//! - `asm!` 内联汇编
//! - 系统调用号（write=1, getpid=39, uname=63）
//!
//! ## x86_64 Linux syscall 约定
//! - rax: 系统调用号
//! - 参数: rdi, rsi, rdx, r10, r8, r9
//! - 返回值: rax
//! - 被破坏的寄存器: rcx, r11

use std::arch::asm;

/// 使用 `write` 系统调用（编号 1）向文件描述符写入数据。
/// 返回写入的字节数，失败返回负数。
///
/// 提示：
/// ```text
/// asm!(
///     "syscall",
///     in("rax") 1u64,        // syscall number for write
///     in("rdi") fd as u64,   // file descriptor
///     in("rsi") buf_ptr,     // buffer pointer
///     in("rdx") buf_len,     // buffer length
///     lateout("rax") ret,    // return value
///     out("rcx") _,          // clobbered by syscall
///     out("r11") _,          // clobbered by syscall
/// )
/// ```
#[cfg(target_os = "linux")]
pub fn sys_write(fd: i32, buf: &[u8]) -> isize {
    // TODO: 使用 asm! 发起 write 系统调用
    todo!()
}

/// 使用 `getpid` 系统调用（编号 39）获取当前进程 ID。
#[cfg(target_os = "linux")]
pub fn sys_getpid() -> i32 {
    // TODO: 使用 asm! 发起 getpid 系统调用
    // getpid 无参数，返回值为进程 ID
    todo!()
}

/// 使用 `write` syscall 向 stdout 打印字符串（自动添加换行）。
#[cfg(target_os = "linux")]
pub fn sys_println(msg: &str) {
    // TODO: 调用 sys_write 向 fd=1 (stdout) 写入 msg
    // TODO: 再写入 "\n"
    todo!()
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use super::*;

    #[test]
    fn test_sys_write_stdout() {
        let msg = b"[sys_write test] hello from raw syscall!\n";
        let ret = sys_write(1, msg);
        assert_eq!(ret, msg.len() as isize);
    }

    #[test]
    fn test_sys_write_stderr() {
        let msg = b"[sys_write test] writing to stderr\n";
        let ret = sys_write(2, msg);
        assert_eq!(ret, msg.len() as isize);
    }

    #[test]
    fn test_sys_getpid() {
        let pid = sys_getpid();
        assert!(pid > 0, "PID should be positive, got {}", pid);
        // 验证与标准库一致
        assert_eq!(pid, std::process::id() as i32);
    }

    #[test]
    fn test_sys_println() {
        // 仅测试不 panic
        sys_println("[sys_println test] hello!");
    }

    #[test]
    fn test_sys_write_empty() {
        let ret = sys_write(1, b"");
        assert_eq!(ret, 0);
    }
}

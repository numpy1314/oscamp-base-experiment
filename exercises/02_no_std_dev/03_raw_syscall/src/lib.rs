//! # Raw System Calls
//!
//! In this exercise, you will use `core::arch::asm!` to make Linux system calls directly,
//! understanding how user-space programs interact with the kernel.
//!
//! ## Key Concepts
//! - x86_64 Linux syscall calling convention
//! - `asm!` inline assembly
//! - System call numbers used here: write=1, getpid=39
//!
//! ## x86_64 Linux Syscall Convention
//! - rax: system call number
//! - arguments: rdi, rsi, rdx, r10, r8, r9
//! - return value: rax
//! - clobbered registers: rcx, r11

use std::arch::asm;

/// Use the `write` system call (number 1) to write data to a file descriptor.
/// Returns the number of bytes written, or a negative value on failure.
///
/// Hint: buffer pointer and length come from `buf.as_ptr()` and `buf.len()`.
/// ```text
/// asm!(
///     "syscall",
///     in("rax") 1u64,        // syscall number for write
///     in("rdi") fd as u64,   // file descriptor
///     in("rsi") buf_ptr,     // buffer pointer (e.g. buf.as_ptr() as u64)
///     in("rdx") buf_len,     // buffer length (e.g. buf.len() as u64)
///     lateout("rax") ret,    // return value (isize)
///     out("rcx") _,          // clobbered by syscall
///     out("r11") _,          // clobbered by syscall
/// )
/// ```
#[cfg(target_os = "linux")]
pub fn sys_write(fd: i32, buf: &[u8]) -> isize {
    todo!()
}

/// Use the `getpid` system call (number 39) to get the current process ID.
/// Hint: no arguments; declare `let ret: i32` and use lateout("rax") ret; include out("rcx") _, out("r11") _.
#[cfg(target_os = "linux")]
pub fn sys_getpid() -> i32 {
    todo!()
}

/// Use the `write` syscall to print a string to stdout (automatically adds newline).
/// Hint: call sys_write(1, msg.as_bytes()) then sys_write(1, b"\n").
#[cfg(target_os = "linux")]
pub fn sys_println(msg: &str) {
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
        // Verify consistency with standard library
        assert_eq!(pid, std::process::id() as i32);
    }

    #[test]
    fn test_sys_println() {
        // Just test that it doesn't panic
        sys_println("[sys_println test] hello!");
    }

    #[test]
    fn test_sys_write_empty() {
        let ret = sys_write(1, b"");
        assert_eq!(ret, 0);
    }
}

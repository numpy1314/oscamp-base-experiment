//! # 文件描述符操作
//!
//! 本练习中，你需要使用原始系统调用操作文件描述符，理解 Unix 文件 I/O 的底层机制。
//!
//! ## 知识点
//! - 文件描述符 (fd) 是 Unix I/O 的核心抽象
//! - open (syscall 2), read (syscall 0), write (syscall 1), close (syscall 3)
//! - O_CREAT, O_WRONLY, O_RDONLY 等标志位
//! - RAII 模式管理文件描述符生命周期
//!
//! ## x86_64 Linux 系统调用号
//! - read: 0
//! - write: 1
//! - open: 2
//! - close: 3

use std::arch::asm;

/// 原始 syscall 辅助函数（3 参数）
#[cfg(target_os = "linux")]
unsafe fn syscall3(nr: u64, a1: u64, a2: u64, a3: u64) -> i64 {
    let ret: i64;
    asm!(
        "syscall",
        in("rax") nr,
        in("rdi") a1,
        in("rsi") a2,
        in("rdx") a3,
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

/// 原始 syscall 辅助函数（1 参数）
#[cfg(target_os = "linux")]
unsafe fn syscall1(nr: u64, a1: u64) -> i64 {
    let ret: i64;
    asm!(
        "syscall",
        in("rax") nr,
        in("rdi") a1,
        lateout("rax") ret,
        out("rcx") _,
        out("r11") _,
    );
    ret
}

const SYS_READ: u64 = 0;
const SYS_WRITE: u64 = 1;
const SYS_OPEN: u64 = 2;
const SYS_CLOSE: u64 = 3;

const O_RDONLY: u64 = 0;
const O_WRONLY: u64 = 1;
const O_CREAT: u64 = 0o100;
const O_TRUNC: u64 = 0o1000;

/// RAII 文件描述符包装器。
/// Drop 时自动关闭 fd。
pub struct FileDesc {
    fd: i32,
}

impl FileDesc {
    /// 用已有的 fd 值创建 FileDesc
    pub fn from_raw(fd: i32) -> Self {
        Self { fd }
    }

    pub fn raw(&self) -> i32 {
        self.fd
    }
}

// TODO: 为 FileDesc 实现 Drop trait
// 调用 close 系统调用关闭文件描述符
#[cfg(target_os = "linux")]
impl Drop for FileDesc {
    fn drop(&mut self) {
        todo!()
    }
}

#[cfg(not(target_os = "linux"))]
impl Drop for FileDesc {
    fn drop(&mut self) {}
}

/// 使用 open 系统调用创建/打开文件用于写入。
/// 返回文件描述符，失败返回 Err(errno)。
///
/// flags = O_WRONLY | O_CREAT | O_TRUNC
/// mode = 0o644
#[cfg(target_os = "linux")]
pub fn open_for_write(path: &str) -> Result<FileDesc, i64> {
    // TODO: 构造 C 风格的路径（末尾加 \0）
    // TODO: 调用 syscall3(SYS_OPEN, path_ptr, flags, mode)
    // TODO: 如果返回值 < 0，返回 Err
    // TODO: 否则返回 Ok(FileDesc::from_raw(fd))
    todo!()
}

/// 使用 open 系统调用打开文件用于读取。
#[cfg(target_os = "linux")]
pub fn open_for_read(path: &str) -> Result<FileDesc, i64> {
    // TODO: 类似 open_for_write，但 flags = O_RDONLY, mode = 0
    todo!()
}

/// 使用 write 系统调用写入数据。
#[cfg(target_os = "linux")]
pub fn fd_write(fd: &FileDesc, buf: &[u8]) -> Result<usize, i64> {
    // TODO: 调用 syscall3(SYS_WRITE, fd, buf_ptr, buf_len)
    // TODO: 返回写入字节数或错误码
    todo!()
}

/// 使用 read 系统调用读取数据。
#[cfg(target_os = "linux")]
pub fn fd_read(fd: &FileDesc, buf: &mut [u8]) -> Result<usize, i64> {
    // TODO: 调用 syscall3(SYS_READ, fd, buf_ptr, buf_len)
    // TODO: 返回读取字节数或错误码
    todo!()
}

#[cfg(test)]
#[cfg(target_os = "linux")]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_write_and_read_file() {
        let path = "/tmp/oscamp_fd_test.txt";
        let content = b"hello from raw fd!";

        // 写入
        {
            let fd = open_for_write(path).expect("open for write failed");
            let n = fd_write(&fd, content).expect("write failed");
            assert_eq!(n, content.len());
            // fd 在此处 drop，自动 close
        }

        // 读取
        {
            let fd = open_for_read(path).expect("open for read failed");
            let mut buf = vec![0u8; 128];
            let n = fd_read(&fd, &mut buf).expect("read failed");
            assert_eq!(&buf[..n], content);
        }

        fs::remove_file(path).ok();
    }

    #[test]
    fn test_open_nonexistent() {
        let result = open_for_read("/tmp/nonexistent_oscamp_file_12345");
        assert!(result.is_err());
    }

    #[test]
    fn test_fd_auto_close() {
        let path = "/tmp/oscamp_fd_close_test.txt";
        let fd_val;
        {
            let fd = open_for_write(path).expect("open failed");
            fd_val = fd.raw();
            // fd drops here
        }
        // 验证 fd 已关闭：尝试写入应失败
        let ret = unsafe { syscall3(SYS_WRITE, fd_val as u64, b"x".as_ptr() as u64, 1) };
        assert!(ret < 0, "fd should be closed after drop");
        fs::remove_file(path).ok();
    }
}

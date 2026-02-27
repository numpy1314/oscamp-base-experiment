//! # File Descriptor Operations
//!
//! In this exercise, you will use raw system calls to operate on file descriptors, understanding the low-level mechanisms of Unix file I/O.
//!
//! ## Concepts
//! - File descriptor (fd) is the core abstraction of Unix I/O
//! - open (syscall 2), read (syscall 0), write (syscall 1), close (syscall 3)
//! - Flags like O_CREAT, O_WRONLY, O_RDONLY
//! - RAII pattern for managing file descriptor lifecycle
//!
//! ## x86_64 Linux System Call Numbers
//! - read: 0
//! - write: 1
//! - open: 2
//! - close: 3

use std::arch::asm;

/// Raw syscall helper function (3 arguments)
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

/// Raw syscall helper function (1 argument)
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

/// RAII file descriptor wrapper.
/// Automatically closes fd when dropped.
pub struct FileDesc {
    fd: i32,
}

impl FileDesc {
    /// Creates a FileDesc from an existing fd value
    pub fn from_raw(fd: i32) -> Self {
        Self { fd }
    }

    pub fn raw(&self) -> i32 {
        self.fd
    }
}

// TODO: Implement Drop trait for FileDesc (only this Linux impl; the other impl is an empty placeholder for non-Linux).
// Call the close system call: unsafe { syscall1(SYS_CLOSE, self.fd as u64); }
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

/// Creates/opens a file for writing using the open system call.
/// Returns a file descriptor, or Err(errno) on failure.
///
/// flags = O_WRONLY | O_CREAT | O_TRUNC
/// mode = 0o644
#[cfg(target_os = "linux")]
pub fn open_for_write(path: &str) -> Result<FileDesc, i64> {
    todo!()
}

/// Opens a file for reading using the open system call.
/// Hint: same as open_for_write but flags = O_RDONLY, mode = 0; path still needs null terminator.
#[cfg(target_os = "linux")]
pub fn open_for_read(path: &str) -> Result<FileDesc, i64> {
    todo!()
}

/// Writes data using the write system call.
/// Hint: syscall3 returns i64; if ret < 0 then Err(ret), else Ok(ret as usize).
#[cfg(target_os = "linux")]
pub fn fd_write(fd: &FileDesc, buf: &[u8]) -> Result<usize, i64> {
    todo!()
}

/// Reads data using the read system call.
/// Hint: same return convention as fd_write: ret < 0 => Err(ret), else Ok(ret as usize).
#[cfg(target_os = "linux")]
pub fn fd_read(fd: &FileDesc, buf: &mut [u8]) -> Result<usize, i64> {
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

        // Write
        {
            let fd = open_for_write(path).expect("open for write failed");
            let n = fd_write(&fd, content).expect("write failed");
            assert_eq!(n, content.len());
            // fd drops here, automatically closes
        }

        // Read
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
        // Verify fd is closed: attempting to write should fail
        let ret = unsafe { syscall3(SYS_WRITE, fd_val as u64, b"x".as_ptr() as u64, 1) };
        assert!(ret < 0, "fd should be closed after drop");
        fs::remove_file(path).ok();
    }
}

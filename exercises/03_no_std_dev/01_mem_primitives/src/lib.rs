//! # no_std 内存操作原语
//!
//! 在 `#![no_std]` 环境下，你没有标准库，只有 `core`。
//! 这些内存操作函数是 OS 内核中最基础的构建块，libc 中的 memcpy/memset 等
//! 函数在裸机环境下需要我们自己实现。
//!
//! ## 任务
//!
//! 实现以下五个函数，要求：
//! - 只能使用 `core` crate，不能使用 `std`
//! - 不能调用 `core::ptr::copy`、`core::ptr::copy_nonoverlapping` 等已有实现（自己写循环）
//! - 正确处理边界情况（n=0、重叠内存区域等）
//! - 通过所有测试

// 生产环境强制 no_std；测试时允许 std（cargo test 的测试框架需要它）
#![cfg_attr(not(test), no_std)]
#![allow(unused_variables)]

/// 将 `src` 起始的 `n` 个字节复制到 `dst`。
///
/// - `dst` 和 `src` 不能重叠（重叠情况请用 `my_memmove`）
/// - 返回 `dst`
///
/// # Safety
/// `dst` 和 `src` 必须各自指向至少 `n` 字节的有效内存。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // TODO: 实现 memcpy
    // 提示：逐字节从 src 读取并写入 dst
    todo!()
}

/// 将 `dst` 起始的 `n` 个字节全部设置为 `c`。
///
/// 返回 `dst`。
///
/// # Safety
/// `dst` 必须指向至少 `n` 字节的有效可写内存。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_memset(dst: *mut u8, c: u8, n: usize) -> *mut u8 {
    // TODO: 实现 memset
    todo!()
}

/// 将 `src` 起始的 `n` 个字节复制到 `dst`，正确处理内存重叠。
///
/// 返回 `dst`。
///
/// # Safety
/// `dst` 和 `src` 必须各自指向至少 `n` 字节的有效内存。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_memmove(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    // TODO: 实现 memmove
    // 提示：当 dst > src 且区域重叠时，需要从后往前复制
    todo!()
}

/// 返回以 null（`\0`）结尾的字节串的长度，不含末尾的 null。
///
/// # Safety
/// `s` 必须指向一个以 null 结尾的有效字节串。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_strlen(s: *const u8) -> usize {
    // TODO: 实现 strlen
    todo!()
}

/// 比较两个以 null 结尾的字节串。
///
/// 返回值：
/// - `0`  : 两串相等
/// - `< 0`: `s1` 在字典序上小于 `s2`
/// - `> 0`: `s1` 在字典序上大于 `s2`
///
/// # Safety
/// `s1` 和 `s2` 必须各自指向以 null 结尾的有效字节串。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn my_strcmp(s1: *const u8, s2: *const u8) -> i32 {
    // TODO: 实现 strcmp
    todo!()
}

// ============================================================
// 测试（#[cfg(test)] 时允许使用 std）
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memcpy_basic() {
        let src = [1u8, 2, 3, 4, 5];
        let mut dst = [0u8; 5];
        unsafe { my_memcpy(dst.as_mut_ptr(), src.as_ptr(), 5) };
        assert_eq!(dst, src);
    }

    #[test]
    fn test_memcpy_zero_len() {
        let src = [0xFFu8; 4];
        let mut dst = [0u8; 4];
        unsafe { my_memcpy(dst.as_mut_ptr(), src.as_ptr(), 0) };
        assert_eq!(dst, [0u8; 4]);
    }

    #[test]
    fn test_memset_basic() {
        let mut buf = [0u8; 8];
        unsafe { my_memset(buf.as_mut_ptr(), 0xAB, 8) };
        assert!(buf.iter().all(|&b| b == 0xAB));
    }

    #[test]
    fn test_memset_partial() {
        let mut buf = [0u8; 8];
        unsafe { my_memset(buf.as_mut_ptr(), 0xFF, 4) };
        assert_eq!(&buf[..4], &[0xFF; 4]);
        assert_eq!(&buf[4..], &[0x00; 4]);
    }

    #[test]
    fn test_memmove_no_overlap() {
        let src = [1u8, 2, 3, 4];
        let mut dst = [0u8; 4];
        unsafe { my_memmove(dst.as_mut_ptr(), src.as_ptr(), 4) };
        assert_eq!(dst, src);
    }

    #[test]
    fn test_memmove_overlap_forward() {
        // 将 buf[0..4] 复制到 buf[1..5]，向右移动 1 位
        let mut buf = [1u8, 2, 3, 4, 5];
        unsafe { my_memmove(buf.as_mut_ptr().add(1), buf.as_ptr(), 4) };
        assert_eq!(buf, [1, 1, 2, 3, 4]);
    }

    #[test]
    fn test_strlen_basic() {
        let s = b"hello\0";
        assert_eq!(unsafe { my_strlen(s.as_ptr()) }, 5);
    }

    #[test]
    fn test_strlen_empty() {
        let s = b"\0";
        assert_eq!(unsafe { my_strlen(s.as_ptr()) }, 0);
    }

    #[test]
    fn test_strcmp_equal() {
        let a = b"hello\0";
        let b = b"hello\0";
        assert_eq!(unsafe { my_strcmp(a.as_ptr(), b.as_ptr()) }, 0);
    }

    #[test]
    fn test_strcmp_less() {
        let a = b"abc\0";
        let b = b"abd\0";
        assert!(unsafe { my_strcmp(a.as_ptr(), b.as_ptr()) } < 0);
    }

    #[test]
    fn test_strcmp_greater() {
        let a = b"abd\0";
        let b = b"abc\0";
        assert!(unsafe { my_strcmp(a.as_ptr(), b.as_ptr()) } > 0);
    }
}

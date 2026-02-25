//! # 全局内存分配器
//!
//! 本练习中，你需要实现一个简单的 Bump 分配器，并理解 `GlobalAlloc` trait 的工作原理。
//!
//! ## 知识点
//! - `std::alloc::GlobalAlloc` trait
//! - 内存对齐（alignment）
//! - 原子操作实现无锁分配
//! - `#[global_allocator]` 属性

use std::alloc::{GlobalAlloc, Layout};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

const HEAP_SIZE: usize = 65536;

#[repr(C, align(4096))]
struct HeapSpace([u8; HEAP_SIZE]);

/// Bump 分配器：只向前分配，不支持单独释放。
/// 这是最简单的分配策略，常用于内核早期启动阶段。
pub struct BumpAllocator {
    heap: UnsafeCell<HeapSpace>,
    next: AtomicUsize,
}

unsafe impl Sync for BumpAllocator {}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            heap: UnsafeCell::new(HeapSpace([0; HEAP_SIZE])),
            next: AtomicUsize::new(0),
        }
    }

    /// 重置分配器，释放所有已分配内存。
    pub fn reset(&self) {
        self.next.store(0, Ordering::Relaxed);
    }

    /// 返回已使用的字节数。
    pub fn used(&self) -> usize {
        self.next.load(Ordering::Relaxed)
    }
}

// TODO: 为 BumpAllocator 实现 GlobalAlloc trait
//
// unsafe fn alloc(&self, layout: Layout) -> *mut u8:
//   1. 获取 heap 的起始地址: self.heap.get() as usize
//   2. 读取 next 偏移量
//   3. 计算对齐后的起始位置:
//      let aligned = (heap_start + next + layout.align() - 1) & !(layout.align() - 1);
//   4. 计算新的 next = aligned - heap_start + layout.size()
//   5. 如果 new_next > HEAP_SIZE，返回 std::ptr::null_mut()
//   6. 更新 self.next（使用 store 即可，单线程测试场景）
//   7. 返回 aligned as *mut u8
//
// unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout):
//   Bump 分配器不支持单独释放，留空即可。
unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator 不支持单独释放
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::Layout;

    static TEST_ALLOCATOR: BumpAllocator = BumpAllocator::new();

    #[test]
    fn test_basic_alloc() {
        TEST_ALLOCATOR.reset();
        let layout = Layout::from_size_align(8, 8).unwrap();
        let ptr = unsafe { TEST_ALLOCATOR.alloc(layout) };
        assert!(!ptr.is_null());
    }

    #[test]
    fn test_alignment() {
        TEST_ALLOCATOR.reset();
        // 先分配 1 字节（对齐 1）
        let layout1 = Layout::from_size_align(1, 1).unwrap();
        unsafe { TEST_ALLOCATOR.alloc(layout1) };

        // 再分配 8 字节（对齐 8）
        let layout2 = Layout::from_size_align(8, 8).unwrap();
        let ptr2 = unsafe { TEST_ALLOCATOR.alloc(layout2) };
        assert!(!ptr2.is_null());
        assert_eq!(ptr2 as usize % 8, 0, "Pointer must be 8-byte aligned");
    }

    #[test]
    fn test_multiple_alloc() {
        TEST_ALLOCATOR.reset();
        let layout = Layout::from_size_align(1024, 8).unwrap();

        let p1 = unsafe { TEST_ALLOCATOR.alloc(layout) };
        let p2 = unsafe { TEST_ALLOCATOR.alloc(layout) };
        assert!(!p1.is_null());
        assert!(!p2.is_null());
        assert_ne!(p1, p2);
        // 两次分配不应重叠
        let diff = (p2 as usize).abs_diff(p1 as usize);
        assert!(diff >= 1024);
    }

    #[test]
    fn test_oom() {
        TEST_ALLOCATOR.reset();
        let layout = Layout::from_size_align(HEAP_SIZE + 1, 1).unwrap();
        let ptr = unsafe { TEST_ALLOCATOR.alloc(layout) };
        assert!(ptr.is_null(), "Should return null when out of memory");
    }

    #[test]
    fn test_used_tracking() {
        TEST_ALLOCATOR.reset();
        assert_eq!(TEST_ALLOCATOR.used(), 0);

        let layout = Layout::from_size_align(64, 8).unwrap();
        unsafe { TEST_ALLOCATOR.alloc(layout) };
        assert!(TEST_ALLOCATOR.used() >= 64);
    }
}

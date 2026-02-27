//! # Bump Allocator（no_std）
//!
//! 实现一个最简单的堆内存分配器：Bump Allocator（碰撞指针分配器）。
//!
//! ## 原理
//!
//! Bump Allocator 维护一个指向"下一个可用地址"的指针 `next`。
//! 每次分配时，将 `next` 对齐到请求的对齐要求，然后向前推进 `size` 字节。
//! 它不支持释放单个对象（`dealloc` 是空操作）。
//!
//! ```text
//! heap_start                              heap_end
//! |----[已分配]----[已分配]----| next |---[空闲]---|
//!                                  ^
//!                              下次从这里开始分配
//! ```
//!
//! ## 任务
//!
//! 实现 `BumpAllocator` 的 `GlobalAlloc::alloc` 方法：
//! 1. 将当前 `next` 向上对齐到 `layout.align()`
//!    提示：`align_up(addr, align) = (addr + align - 1) & !(align - 1)`
//! 2. 检查对齐后的地址加上 `layout.size()` 是否超出 `heap_end`
//! 3. 超出时返回 `null_mut()`，否则用 `compare_exchange` 原子更新 `next` 并返回地址
//!
//! ## 关键知识点
//!
//! - `core::alloc::{GlobalAlloc, Layout}`
//! - 内存对齐计算
//! - `AtomicUsize` 与 `compare_exchange`（CAS 循环）

#![cfg_attr(not(test), no_std)]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: AtomicUsize,
}

impl BumpAllocator {
    /// 创建一个新的 BumpAllocator。
    ///
    /// # Safety
    /// `heap_start..heap_end` 必须是一段有效的、可读写的内存区域，
    /// 且在该 allocator 的生命周期内不被其他代码使用。
    pub const unsafe fn new(heap_start: usize, heap_end: usize) -> Self {
        Self {
            heap_start,
            heap_end,
            next: AtomicUsize::new(heap_start),
        }
    }

    /// 重置分配器（释放所有已分配内存）。
    pub fn reset(&self) {
        self.next.store(self.heap_start, Ordering::SeqCst);
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // TODO: 实现 bump 分配
        //
        // 步骤：
        // 1. 读取当前 next（使用 Ordering::SeqCst）
        // 2. 将 next 向上对齐到 layout.align()
        //    提示：align_up(addr, align) = (addr + align - 1) & !(align - 1)
        // 3. 计算分配结束地址 = aligned + layout.size()
        // 4. 如果结束地址 > heap_end，返回 null_mut()
        // 5. 用 compare_exchange 原子地更新 next 到结束地址
        //    （若 CAS 失败说明有并发，需要重试——用循环）
        // 6. 返回 aligned 地址对应的指针
        todo!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator 不回收单个对象，留空即可
    }
}

// ============================================================
// 测试
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;

    const HEAP_SIZE: usize = 4096;

    fn make_allocator() -> (BumpAllocator, Vec<u8>) {
        let mut heap = vec![0u8; HEAP_SIZE];
        let start = heap.as_mut_ptr() as usize;
        let alloc = unsafe { BumpAllocator::new(start, start + HEAP_SIZE) };
        (alloc, heap)
    }

    #[test]
    fn test_alloc_basic() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(16, 8).unwrap();
        let ptr = unsafe { alloc.alloc(layout) };
        assert!(!ptr.is_null(), "分配应成功");
    }

    #[test]
    fn test_alloc_alignment() {
        let (alloc, _heap) = make_allocator();
        for align in [1, 2, 4, 8, 16, 64] {
            let layout = Layout::from_size_align(1, align).unwrap();
            let ptr = unsafe { alloc.alloc(layout) };
            assert!(!ptr.is_null());
            assert_eq!(
                ptr as usize % align,
                0,
                "返回地址必须满足 align={align} 的对齐要求"
            );
        }
    }

    #[test]
    fn test_alloc_no_overlap() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(64, 8).unwrap();
        let p1 = unsafe { alloc.alloc(layout) } as usize;
        let p2 = unsafe { alloc.alloc(layout) } as usize;
        assert!(p1 + 64 <= p2 || p2 + 64 <= p1, "两次分配不能重叠");
    }

    #[test]
    fn test_alloc_oom() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(HEAP_SIZE + 1, 1).unwrap();
        let ptr = unsafe { alloc.alloc(layout) };
        assert!(ptr.is_null(), "超出堆范围时应返回 null");
    }

    #[test]
    fn test_alloc_fill_heap() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(256, 1).unwrap();
        for i in 0..16 {
            let ptr = unsafe { alloc.alloc(layout) };
            assert!(!ptr.is_null(), "第 {i} 次分配应成功");
        }
        let ptr = unsafe { alloc.alloc(layout) };
        assert!(ptr.is_null(), "堆满后应返回 null");
    }

    #[test]
    fn test_reset() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(HEAP_SIZE, 1).unwrap();
        let p1 = unsafe { alloc.alloc(layout) };
        assert!(!p1.is_null());
        alloc.reset();
        let p2 = unsafe { alloc.alloc(layout) };
        assert!(!p2.is_null(), "reset 后应能重新分配");
        assert_eq!(p1, p2, "reset 后分配地址应与第一次相同");
    }
}

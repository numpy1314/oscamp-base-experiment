//! # Global Memory Allocator
//!
//! In this exercise, you will implement a simple Bump allocator and understand how the `GlobalAlloc` trait works.
//!
//! ## Concepts
//! - `std::alloc::GlobalAlloc` trait
//! - Memory alignment (alignment)
//! - Atomic operations for lock‑free allocation
//! - `#[global_allocator]` attribute

use std::alloc::{GlobalAlloc, Layout};
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

const HEAP_SIZE: usize = 65536;

#[repr(C, align(4096))]
struct HeapSpace([u8; HEAP_SIZE]);

/// Bump allocator: only allocates forward, does not support individual deallocation.
/// This is the simplest allocation strategy, often used in early kernel boot phase.
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

    /// Reset the allocator, freeing all allocated memory.
    pub fn reset(&self) {
        self.next.store(0, Ordering::Relaxed);
    }

    /// Returns number of bytes used.
    pub fn used(&self) -> usize {
        self.next.load(Ordering::Relaxed)
    }
}

// TODO: Implement GlobalAlloc trait for BumpAllocator
//
// unsafe fn alloc(&self, layout: Layout) -> *mut u8:
//   1. Get heap start address: self.heap.get() as usize
//   2. Read next offset
//   3. Compute aligned start position:
//      let aligned = (heap_start + next + layout.align() - 1) & !(layout.align() - 1);
//   4. Compute new next = aligned - heap_start + layout.size()
//   5. If new_next > HEAP_SIZE, return std::ptr::null_mut()
//   6. Update self.next (store is enough for single‑threaded test scenario)
//   7. Return aligned as *mut u8
//
// unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout):
//   Bump allocator does not support individual deallocation, leave empty.
unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator does not support individual deallocation
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
        // First allocate 1 byte (alignment 1)
        let layout1 = Layout::from_size_align(1, 1).unwrap();
        unsafe { TEST_ALLOCATOR.alloc(layout1) };

        // Then allocate 8 bytes (alignment 8)
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
        // Two allocations should not overlap
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

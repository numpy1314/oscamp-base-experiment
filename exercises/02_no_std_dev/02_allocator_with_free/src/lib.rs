//! # Allocator with Allocation and Deallocation
//!
//! This exercise implements a **free-list allocator** that supports both allocation and deallocation,
//! so that freed memory can be reused. This is the natural follow-up to the Bump allocator (without free):
//! here you will manage a linked list of free blocks and implement `alloc` / `dealloc`.
//!
//! ## Concepts
//! - Free list: a linked list of free blocks; alloc takes from the list, dealloc returns to the list
//! - Block header: each block has a header (e.g. size) so that on dealloc we know how many bytes to reclaim
//! - **Algorithm choice**: The reference solution uses **first-fit** (take the first block that fits).
//!   You may instead implement **best-fit**, **coalescing** of adjacent free blocks on dealloc, or other
//!   strategies—any correct allocator that passes the tests is acceptable.
//!
//! ## Block layout (design used in tests)
//! - Each block: `[size: usize][payload...]`. The pointer returned to the user points to the payload.
//! - When the block is free, the first 8 bytes of the payload area store the `next` pointer (free list link).
//! - Minimum block size is 16 bytes (8-byte header + 8-byte next or payload).

use std::alloc::{GlobalAlloc, Layout};
use std::cell::UnsafeCell;
use std::ptr::null_mut;

const HEAP_SIZE: usize = 65536;
/// Minimum block size: header (8) + at least 8 bytes for next pointer or payload.
const MIN_BLOCK: usize = 16;

#[repr(C, align(4096))]
struct HeapSpace([u8; HEAP_SIZE]);

/// Free-list allocator: supports both alloc and dealloc; freed blocks are reused.
pub struct FreeListAllocator {
    heap: UnsafeCell<HeapSpace>,
    /// Head of the free list: pointer (as usize) to the first free block, or 0 for empty.
    free_head: UnsafeCell<usize>,
}

unsafe impl Sync for FreeListAllocator {}

impl FreeListAllocator {
    pub fn new() -> Self {
        let alloc = Self {
            heap: UnsafeCell::new(HeapSpace([0; HEAP_SIZE])),
            free_head: UnsafeCell::new(0),
        };
        alloc.init_heap();
        alloc
    }

    /// Initialize the heap with one big free block covering the entire heap.
    fn init_heap(&self) {
        let heap_start = self.heap.get() as usize;
        unsafe {
            // One free block: size = HEAP_SIZE, next = null
            *(heap_start as *mut usize) = HEAP_SIZE;
            *((heap_start + 8) as *mut usize) = 0;
            *self.free_head.get() = heap_start;
        }
    }

    /// Reset the allocator to initial state (one big free block). Used by tests.
    pub fn reset(&self) {
        self.init_heap();
    }

    /// Read head of free list.
    fn free_head(&self) -> usize {
        unsafe { *self.free_head.get() }
    }

    /// Set head of free list.
    fn set_free_head(&self, ptr: usize) {
        unsafe { *self.free_head.get() = ptr };
    }

    /// Compute total block size needed for a given layout (header + aligned payload).
    /// Used by both alloc and by tests to reason about splitting.
    pub fn block_size_for(layout: Layout) -> usize {
        let payload_size = (layout.size() + layout.align().wrapping_sub(1)) & !(layout.align().wrapping_sub(1));
        let total = 8 + payload_size;
        if total < MIN_BLOCK {
            MIN_BLOCK
        } else {
            total
        }
    }
}

// TODO: Implement GlobalAlloc for FreeListAllocator
//
// unsafe fn alloc(&self, layout: Layout) -> *mut u8:
//   (Reference: first-fit; you may use other strategies such as best-fit.)
//   1. Compute need = Self::block_size_for(layout).
//   2. Traverse the free list (starting from self.free_head()); for each block, read size at block start.
//   3. Find the first block with size >= need. Unlink it from the list (update previous node's next, or free_head).
//   4. If block_size - need >= MIN_BLOCK, split: the remainder (at block + need) forms a new free block;
//      set its size and next, insert it at the head of the free list.
//   5. Write need at the chosen block's start (header). Return (block + 8) as *mut u8.
//   6. If no block fits, return null_mut().
//
// unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout):
//   1. If ptr.is_null(), return.
//   2. block_start = (ptr as usize) - 8. Read block_size from *(block_start as *const usize).
//   3. Insert this block at the head of the free list: *(block_start as *mut usize) = block_size;
//      *((block_start + 8) as *mut usize) = self.free_head(); self.set_free_head(block_start).
//
unsafe impl GlobalAlloc for FreeListAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::Layout;

    fn allocator() -> FreeListAllocator {
        FreeListAllocator::new()
    }

    /// Single round: alloc several blocks, free one, alloc again should reuse the freed block.
    #[test]
    fn test_single_round_alloc_free() {
        let alloc = allocator();
        let layout = Layout::from_size_align(64, 8).unwrap();

        let p1 = unsafe { alloc.alloc(layout) };
        let p2 = unsafe { alloc.alloc(layout) };
        let p3 = unsafe { alloc.alloc(layout) };
        assert!(!p1.is_null() && !p2.is_null() && !p3.is_null());
        assert!(p1 != p2 && p2 != p3 && p1 != p3);

        // Free middle one, then allocate again — should reuse
        unsafe { alloc.dealloc(p2, layout) };
        let p4 = unsafe { alloc.alloc(layout) };
        assert!(!p4.is_null());
        assert_eq!(p4, p2, "alloc after free should reuse the freed block");

        unsafe { alloc.dealloc(p1, layout) };
        unsafe { alloc.dealloc(p3, layout) };
        unsafe { alloc.dealloc(p4, layout) };
    }

    /// Multi-round: each round alloc many blocks, then free all in order.
    #[test]
    fn test_multi_round_batch_alloc_batch_free() {
        let alloc = allocator();
        let layout = Layout::from_size_align(128, 8).unwrap();
        const ROUNDS: usize = 5;
        const PER_ROUND: usize = 20;

        for _ in 0..ROUNDS {
            let mut ptrs = Vec::new();
            for _ in 0..PER_ROUND {
                let p = unsafe { alloc.alloc(layout) };
                assert!(!p.is_null(), "alloc should succeed in batch");
                ptrs.push(p);
            }
            for p in ptrs {
                unsafe { alloc.dealloc(p, layout) };
            }
        }
    }

    /// Multi-round: interleaved alloc and free (same layout).
    #[test]
    fn test_multi_round_interleaved_alloc_free() {
        let alloc = allocator();
        let layout = Layout::from_size_align(64, 8).unwrap();

        let mut held = Vec::new();
        for i in 0..50 {
            if i % 3 == 0 && !held.is_empty() {
                let p = held.pop().unwrap();
                unsafe { alloc.dealloc(p, layout) };
            }
            let p = unsafe { alloc.alloc(layout) };
            assert!(!p.is_null());
            held.push(p);
        }
        for p in held {
            unsafe { alloc.dealloc(p, layout) };
        }
    }

    #[test]
    fn test_alignment() {
        let alloc = allocator();
        alloc.reset();
        let _ = unsafe { alloc.alloc(Layout::from_size_align(1, 1).unwrap()) };
        let p = unsafe { alloc.alloc(Layout::from_size_align(8, 8).unwrap()) };
        assert!(!p.is_null());
        assert_eq!((p as usize) % 8, 0);
    }

    #[test]
    fn test_oom_returns_null() {
        let alloc = allocator();
        let layout = Layout::from_size_align(HEAP_SIZE + 1, 1).unwrap();
        let p = unsafe { alloc.alloc(layout) };
        assert!(p.is_null());
    }

    #[test]
    fn test_block_size_for() {
        assert_eq!(FreeListAllocator::block_size_for(Layout::from_size_align(1, 1).unwrap()), 16);
        assert_eq!(FreeListAllocator::block_size_for(Layout::from_size_align(64, 8).unwrap()), 72);
    }
}

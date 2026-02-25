//! # Mutex 共享状态
//!
//! 本练习中，你需要使用 `Arc<Mutex<T>>` 在多线程间安全地共享和修改数据。
//!
//! ## 知识点
//! - `Mutex<T>` 互斥锁保护共享数据
//! - `Arc<T>` 原子引用计数实现跨线程共享
//! - `lock()` 获取锁并访问数据

use std::sync::{Arc, Mutex};
use std::thread;

/// 使用 `n_threads` 个线程并发地对计数器进行递增操作。
/// 每个线程将计数器增加 `count_per_thread` 次。
/// 返回最终的计数器值。
///
/// 提示：使用 `Arc<Mutex<usize>>` 作为共享计数器。
pub fn concurrent_counter(n_threads: usize, count_per_thread: usize) -> usize {
    // TODO: 创建 Arc<Mutex<usize>> 初始值为 0
    // TODO: 启动 n_threads 个线程
    // TODO: 每个线程中 lock() 并递增 count_per_thread 次
    // TODO: join 所有线程，返回最终值
    todo!()
}

/// 使用多线程并发地向共享向量中添加元素。
/// 每个线程将自己的 id (0..n_threads) 添加到向量中。
/// 返回排序后的向量。
///
/// 提示：使用 `Arc<Mutex<Vec<usize>>>`。
pub fn concurrent_collect(n_threads: usize) -> Vec<usize> {
    // TODO: 创建 Arc<Mutex<Vec<usize>>>
    // TODO: 每个线程 push 自己的 id
    // TODO: join 所有线程后，对结果排序并返回
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_single_thread() {
        assert_eq!(concurrent_counter(1, 100), 100);
    }

    #[test]
    fn test_counter_multi_thread() {
        assert_eq!(concurrent_counter(10, 100), 1000);
    }

    #[test]
    fn test_counter_zero() {
        assert_eq!(concurrent_counter(5, 0), 0);
    }

    #[test]
    fn test_collect() {
        let result = concurrent_collect(5);
        assert_eq!(result, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_collect_single() {
        assert_eq!(concurrent_collect(1), vec![0]);
    }
}

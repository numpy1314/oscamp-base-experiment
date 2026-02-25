//! # 线程创建
//!
//! 本练习中，你需要学习如何创建线程并在线程间传递数据。
//!
//! ## 知识点
//! - `std::thread::spawn` 创建新线程
//! - `move` 闭包捕获变量所有权
//! - `JoinHandle::join()` 等待线程完成并获取返回值

use std::thread;

/// 在新线程中将向量的每个元素乘以 2，返回结果向量。
///
/// 提示：使用 `thread::spawn` 和 `move` 闭包。
pub fn double_in_thread(numbers: Vec<i32>) -> Vec<i32> {
    // TODO: 创建一个新线程，将 numbers 中每个元素乘以 2
    // 使用 thread::spawn 和 move 闭包
    // 使用 join().unwrap() 获取结果
    todo!()
}

/// 并行地对两个向量分别求和，返回两个和的元组。
///
/// 提示：创建两个线程分别计算。
pub fn parallel_sum(a: Vec<i32>, b: Vec<i32>) -> (i32, i32) {
    // TODO: 创建两个线程分别对 a 和 b 求和
    // 分别 join 两个线程获取结果
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_basic() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(double_in_thread(nums), vec![2, 4, 6, 8, 10]);
    }

    #[test]
    fn test_double_empty() {
        assert_eq!(double_in_thread(vec![]), vec![]);
    }

    #[test]
    fn test_double_negative() {
        assert_eq!(double_in_thread(vec![-1, 0, 1]), vec![-2, 0, 2]);
    }

    #[test]
    fn test_parallel_sum() {
        let a = vec![1, 2, 3];
        let b = vec![10, 20, 30];
        assert_eq!(parallel_sum(a, b), (6, 60));
    }

    #[test]
    fn test_parallel_sum_empty() {
        assert_eq!(parallel_sum(vec![], vec![]), (0, 0));
    }
}

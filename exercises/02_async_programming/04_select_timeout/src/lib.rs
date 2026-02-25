//! # Select 与超时
//!
//! 本练习中，你需要使用 `tokio::select!` 宏实现竞态选择和超时控制。
//!
//! ## 知识点
//! - `tokio::select!` 同时等待多个异步操作
//! - `tokio::time::timeout` 超时控制
//! - 第一个完成的分支被执行，其余被取消

use std::future::Future;
use tokio::time::{sleep, Duration};

/// 带超时的异步操作。
/// 如果 `future` 在 `timeout_ms` 毫秒内完成，返回 Some(结果)。
/// 否则返回 None。
///
/// 提示：使用 `tokio::select!` 或 `tokio::time::timeout`。
pub async fn with_timeout<F, T>(future: F, timeout_ms: u64) -> Option<T>
where
    F: Future<Output = T>,
{
    // TODO: 使用 tokio::select! 在 future 和 sleep 之间竞争
    // 或者使用 tokio::time::timeout
    todo!()
}

/// 竞速执行两个异步任务，返回先完成的那个结果。
///
/// 提示：使用 `tokio::select!` 宏。
pub async fn race<F1, F2, T>(f1: F1, f2: F2) -> T
where
    F1: Future<Output = T>,
    F2: Future<Output = T>,
{
    // TODO: 使用 tokio::select! 等待 f1 和 f2
    // 返回先完成的结果
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_timeout_success() {
        let result = with_timeout(async { 42 }, 100).await;
        assert_eq!(result, Some(42));
    }

    #[tokio::test]
    async fn test_timeout_expired() {
        let result = with_timeout(async {
            sleep(Duration::from_millis(200)).await;
            42
        }, 50).await;
        assert_eq!(result, None);
    }

    #[tokio::test]
    async fn test_race_first_wins() {
        let result = race(
            async {
                sleep(Duration::from_millis(10)).await;
                "fast"
            },
            async {
                sleep(Duration::from_millis(200)).await;
                "slow"
            },
        ).await;
        assert_eq!(result, "fast");
    }

    #[tokio::test]
    async fn test_race_second_wins() {
        let result = race(
            async {
                sleep(Duration::from_millis(200)).await;
                "slow"
            },
            async {
                sleep(Duration::from_millis(10)).await;
                "fast"
            },
        ).await;
        assert_eq!(result, "fast");
    }
}

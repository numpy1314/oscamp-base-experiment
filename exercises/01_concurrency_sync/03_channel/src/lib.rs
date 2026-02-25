//! # Channel 通道通信
//!
//! 本练习中，你需要使用 `std::sync::mpsc` 通道在线程间传递消息。
//!
//! ## 知识点
//! - `mpsc::channel()` 创建多生产者单消费者通道
//! - `Sender::send()` 发送消息
//! - `Receiver::recv()` 接收消息
//! - 多个生产者可通过 `Sender::clone()` 创建

use std::sync::mpsc;
use std::thread;

/// 创建一个生产者线程，将 items 中的元素逐个发送到通道。
/// 主线程接收所有消息并返回。
pub fn simple_send_recv(items: Vec<String>) -> Vec<String> {
    // TODO: 创建 channel
    // TODO: spawn 线程发送 items 中每个元素
    // TODO: 在主线程中接收所有消息并收集到 Vec
    // 提示：当所有 Sender 被 drop 后，recv() 会返回 Err
    todo!()
}

/// 创建 `n_producers` 个生产者线程，每个线程发送 `"msg from {id}"` 格式的消息。
/// 收集所有消息，按字典序排序后返回。
///
/// 提示：使用 `tx.clone()` 创建多个发送端。注意原始 tx 也需要被 drop。
pub fn multi_producer(n_producers: usize) -> Vec<String> {
    // TODO: 创建 channel
    // TODO: 为每个生产者 clone 一个 sender
    // TODO: 注意 drop 原始 sender，否则 receiver 不会结束
    // TODO: 收集所有消息并排序
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_send_recv() {
        let items = vec!["hello".into(), "world".into(), "rust".into()];
        let result = simple_send_recv(items.clone());
        assert_eq!(result, items);
    }

    #[test]
    fn test_simple_empty() {
        let result = simple_send_recv(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_multi_producer() {
        let result = multi_producer(3);
        assert_eq!(result, vec![
            "msg from 0".to_string(),
            "msg from 1".to_string(),
            "msg from 2".to_string(),
        ]);
    }

    #[test]
    fn test_multi_producer_single() {
        let result = multi_producer(1);
        assert_eq!(result, vec!["msg from 0".to_string()]);
    }
}

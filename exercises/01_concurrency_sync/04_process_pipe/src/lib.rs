//! # 进程与管道
//!
//! 本练习中，你需要学习如何创建子进程并通过管道进行通信。
//!
//! ## 知识点
//! - `std::process::Command` 创建子进程
//! - `Stdio::piped()` 设置管道
//! - 通过 stdin/stdout 与子进程通信
//! - 获取子进程退出状态

use std::io::{Read, Write};
use std::process::{Command, Stdio};

/// 执行给定的 shell 命令并返回其 stdout 输出。
///
/// 例如：`run_command("echo", &["hello"])` 应返回 `"hello\n"`
pub fn run_command(program: &str, args: &[&str]) -> String {
    // TODO: 使用 Command::new 创建进程
    // TODO: 设置 stdout 为 Stdio::piped()
    // TODO: 用 .output() 执行并获取输出
    // TODO: 将 stdout 转为 String 返回
    todo!()
}

/// 通过管道向子进程 (cat) 的 stdin 写入数据，并读取其 stdout 输出。
///
/// 这演示了父子进程间的双向管道通信。
pub fn pipe_through_cat(input: &str) -> String {
    // TODO: 创建 "cat" 命令，设置 stdin 和 stdout 为 piped
    // TODO: spawn 进程
    // TODO: 向子进程 stdin 写入 input
    // TODO: drop stdin 以关闭管道（否则 cat 不会退出）
    // TODO: 从子进程 stdout 读取输出
    todo!()
}

/// 获取子进程的退出码。
/// 执行命令 `sh -c {command}` 并返回退出码。
pub fn get_exit_code(command: &str) -> i32 {
    // TODO: 使用 Command::new("sh").args(["-c", command])
    // TODO: 执行并获取 status
    // TODO: 返回 exit code
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_echo() {
        let output = run_command("echo", &["hello"]);
        assert_eq!(output.trim(), "hello");
    }

    #[test]
    fn test_run_with_args() {
        let output = run_command("echo", &["-n", "no newline"]);
        assert_eq!(output, "no newline");
    }

    #[test]
    fn test_pipe_cat() {
        let output = pipe_through_cat("hello pipe!");
        assert_eq!(output, "hello pipe!");
    }

    #[test]
    fn test_pipe_multiline() {
        let input = "line1\nline2\nline3";
        assert_eq!(pipe_through_cat(input), input);
    }

    #[test]
    fn test_exit_code_success() {
        assert_eq!(get_exit_code("true"), 0);
    }

    #[test]
    fn test_exit_code_failure() {
        assert_eq!(get_exit_code("false"), 1);
    }
}

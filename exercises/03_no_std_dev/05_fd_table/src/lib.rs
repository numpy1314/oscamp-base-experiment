//! # 文件描述符表
//!
//! 实现一个简单的文件描述符（fd）表——OS 内核中管理打开文件的核心数据结构。
//!
//! ## 背景
//!
//! 在 Linux 内核中，每个进程都有一张 fd 表，将整数 fd 映射到内核文件对象。
//! 用户程序通过 fd 进行 read/write/close 等操作，内核通过 fd 表找到对应的文件对象。
//!
//! ```text
//! fd 表:
//!   0 -> Stdin
//!   1 -> Stdout
//!   2 -> Stderr
//!   3 -> File("/etc/passwd")
//!   4 -> (空)
//!   5 -> Socket(...)
//! ```
//!
//! ## 任务
//!
//! 实现 `FdTable` 的以下方法：
//!
//! - `new()` → 创建空 fd 表
//! - `alloc(file)` → `usize`：分配一个新 fd，返回 fd 编号
//!   - 优先复用已关闭的最小 fd 编号
//!   - 若无空位则扩展表
//! - `get(fd)` → `Option<Arc<dyn File>>`：获取 fd 对应的文件对象
//! - `close(fd)` → `bool`：关闭 fd，返回是否成功（fd 不存在则返回 false）
//! - `count()` → `usize`：返回当前已分配的 fd 数量（不含已关闭的）
//!
//! ## 关键知识点
//!
//! - `trait object`：`Arc<dyn File>`
//! - `Vec<Option<T>>` 作为稀疏表
//! - fd 编号复用策略（找最小空闲位）
//! - `Arc` 的引用计数与资源释放

use std::sync::Arc;

/// 文件抽象 trait，内核中所有"文件"（普通文件、管道、socket）都实现此接口
pub trait File: Send + Sync {
    fn read(&self, buf: &mut [u8]) -> isize;
    fn write(&self, buf: &[u8]) -> isize;
}

/// 文件描述符表
pub struct FdTable {
    // TODO: 自行设计内部结构
    // 提示：可以用 Vec<Option<Arc<dyn File>>>
    //       索引即为 fd 编号，None 表示该 fd 已关闭或未分配
}

impl FdTable {
    /// 创建一个空的 fd 表
    pub fn new() -> Self {
        // TODO
        todo!()
    }

    /// 分配一个新 fd，返回 fd 编号。
    ///
    /// 优先复用已关闭的最小 fd 编号；若无空位则追加到末尾。
    pub fn alloc(&mut self, file: Arc<dyn File>) -> usize {
        // TODO
        todo!()
    }

    /// 获取 fd 对应的文件对象。fd 不存在或已关闭时返回 None。
    pub fn get(&self, fd: usize) -> Option<Arc<dyn File>> {
        // TODO
        todo!()
    }

    /// 关闭 fd。成功返回 true，fd 不存在或已关闭返回 false。
    pub fn close(&mut self, fd: usize) -> bool {
        // TODO
        todo!()
    }

    /// 返回当前已分配的 fd 数量（不含已关闭的）
    pub fn count(&self) -> usize {
        // TODO
        todo!()
    }
}

impl Default for FdTable {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================
// 测试用的 File 实现
// ============================================================
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct MockFile {
        id: usize,
        write_log: Mutex<Vec<Vec<u8>>>,
    }

    impl MockFile {
        fn new(id: usize) -> Arc<Self> {
            Arc::new(Self { id, write_log: Mutex::new(vec![]) })
        }
    }

    impl File for MockFile {
        fn read(&self, buf: &mut [u8]) -> isize {
            buf[0] = self.id as u8;
            1
        }
        fn write(&self, buf: &[u8]) -> isize {
            self.write_log.lock().unwrap().push(buf.to_vec());
            buf.len() as isize
        }
    }

    #[test]
    fn test_alloc_basic() {
        let mut table = FdTable::new();
        let fd = table.alloc(MockFile::new(0));
        assert_eq!(fd, 0, "第一个 fd 应为 0");
        let fd2 = table.alloc(MockFile::new(1));
        assert_eq!(fd2, 1, "第二个 fd 应为 1");
    }

    #[test]
    fn test_get() {
        let mut table = FdTable::new();
        let file = MockFile::new(42);
        let fd = table.alloc(file);
        let got = table.get(fd);
        assert!(got.is_some(), "get 应返回 Some");
        let mut buf = [0u8; 1];
        got.unwrap().read(&mut buf);
        assert_eq!(buf[0], 42);
    }

    #[test]
    fn test_get_invalid() {
        let table = FdTable::new();
        assert!(table.get(0).is_none());
        assert!(table.get(999).is_none());
    }

    #[test]
    fn test_close_and_reuse() {
        let mut table = FdTable::new();
        let fd0 = table.alloc(MockFile::new(0)); // fd=0
        let fd1 = table.alloc(MockFile::new(1)); // fd=1
        let fd2 = table.alloc(MockFile::new(2)); // fd=2

        assert!(table.close(fd1), "关闭 fd=1 应成功");
        assert!(table.get(fd1).is_none(), "关闭后 get 应返回 None");

        // 再分配应复用 fd=1（最小空闲）
        let fd_new = table.alloc(MockFile::new(99));
        assert_eq!(fd_new, fd1, "应复用已关闭的最小 fd");

        let _ = (fd0, fd2);
    }

    #[test]
    fn test_close_invalid() {
        let mut table = FdTable::new();
        assert!(!table.close(0), "关闭不存在的 fd 应返回 false");
    }

    #[test]
    fn test_count() {
        let mut table = FdTable::new();
        assert_eq!(table.count(), 0);
        let fd0 = table.alloc(MockFile::new(0));
        let fd1 = table.alloc(MockFile::new(1));
        assert_eq!(table.count(), 2);
        table.close(fd0);
        assert_eq!(table.count(), 1);
        table.close(fd1);
        assert_eq!(table.count(), 0);
    }

    #[test]
    fn test_write_through_fd() {
        let mut table = FdTable::new();
        let file = MockFile::new(0);
        let fd = table.alloc(file);
        let f = table.get(fd).unwrap();
        let n = f.write(b"hello");
        assert_eq!(n, 5);
    }
}

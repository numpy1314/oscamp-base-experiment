//! # 页表项标志位
//!
//! 本练习中，你需要学习 RISC-V SV39 页表项（Page Table Entry）的结构，
//! 并通过位操作来构造和解析页表项。
//!
//! ## 知识点
//! - RISC-V SV39 页表项的 64 位布局
//! - 位运算构造/提取字段
//! - 页表项标志位的含义（V/R/W/X/U/G/A/D）
//!
//! ## SV39 PTE 布局（64 位）
//! ```text
//! 63    54 53        10 9  8 7 6 5 4 3 2 1 0
//! ┌───────┬────────────┬────┬─┬─┬─┬─┬─┬─┬─┬─┐
//! │ Rsvd  │  PPN[2:0]  │ RSW│D│A│G│U│X│W│R│V│
//! │ 10bit │  44 bits   │ 2b │ │ │ │ │ │ │ │ │
//! └───────┴────────────┴────┴─┴─┴─┴─┴─┴─┴─┴─┘
//! ```

/// 页表项标志位常量
pub const PTE_V: u64 = 1 << 0; // Valid
pub const PTE_R: u64 = 1 << 1; // Readable
pub const PTE_W: u64 = 1 << 2; // Writable
pub const PTE_X: u64 = 1 << 3; // Executable
pub const PTE_U: u64 = 1 << 4; // User accessible
pub const PTE_G: u64 = 1 << 5; // Global
pub const PTE_A: u64 = 1 << 6; // Accessed
pub const PTE_D: u64 = 1 << 7; // Dirty

/// PPN 字段在 PTE 中的位移和掩码
const PPN_SHIFT: u32 = 10;
const PPN_MASK: u64 = (1u64 << 44) - 1; // 44 位 PPN

/// 根据物理页号 (PPN) 和标志位构造一个页表项。
///
/// PPN 占据 bits [53:10]，标志位占据 bits [7:0]。
///
/// 示例：ppn=0x12345, flags=PTE_V|PTE_R|PTE_W
/// 结果应为：(0x12345 << 10) | 0b111 = 0x48D14007
///
/// 提示：将 PPN 左移 PPN_SHIFT 位，然后与 flags 做按位或。
pub fn make_pte(ppn: u64, flags: u64) -> u64 {
    // TODO: 用 ppn 和 flags 构造页表项
    todo!()
}

/// 从页表项中提取物理页号 (PPN)。
///
/// 提示：右移 PPN_SHIFT 位后，用 PPN_MASK 做按位与。
pub fn extract_ppn(pte: u64) -> u64 {
    // TODO: 从 pte 中提取 PPN
    todo!()
}

/// 从页表项中提取标志位（低 8 位）。
pub fn extract_flags(pte: u64) -> u64 {
    // TODO: 提取低 8 位标志
    todo!()
}

/// 检查页表项是否有效（V 位是否置位）。
pub fn is_valid(pte: u64) -> bool {
    // TODO: 检查 PTE_V
    todo!()
}

/// 判断页表项是否为叶节点（leaf PTE）。
///
/// 在 SV39 中，如果 R、W、X 中任意一位被置位，该 PTE 就是叶节点，
/// 指向最终的物理页。否则它指向下一级页表。
pub fn is_leaf(pte: u64) -> bool {
    // TODO: 检查 R/W/X 中是否有任意一位被置位
    todo!()
}

/// 根据给定的权限检查页表项是否允许该访问。
///
/// - `read`: 需要可读权限
/// - `write`: 需要可写权限
/// - `execute`: 需要可执行权限
///
/// 返回 true 当且仅当：PTE 有效，且所需的每种权限都被满足。
pub fn check_permission(pte: u64, read: bool, write: bool, execute: bool) -> bool {
    // TODO: 先检查是否 valid，再逐一检查所需权限
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_pte_basic() {
        let pte = make_pte(0x12345, PTE_V | PTE_R | PTE_W);
        assert_eq!(extract_ppn(pte), 0x12345);
        assert_eq!(extract_flags(pte), PTE_V | PTE_R | PTE_W);
    }

    #[test]
    fn test_make_pte_zero() {
        let pte = make_pte(0, 0);
        assert_eq!(pte, 0);
        assert_eq!(extract_ppn(pte), 0);
        assert_eq!(extract_flags(pte), 0);
    }

    #[test]
    fn test_make_pte_all_flags() {
        let all = PTE_V | PTE_R | PTE_W | PTE_X | PTE_U | PTE_G | PTE_A | PTE_D;
        let pte = make_pte(0xABC, all);
        assert_eq!(extract_ppn(pte), 0xABC);
        assert_eq!(extract_flags(pte), all);
    }

    #[test]
    fn test_make_pte_large_ppn() {
        let ppn = (1u64 << 44) - 1; // 最大 PPN
        let pte = make_pte(ppn, PTE_V);
        assert_eq!(extract_ppn(pte), ppn);
    }

    #[test]
    fn test_is_valid() {
        assert!(is_valid(make_pte(1, PTE_V)));
        assert!(!is_valid(make_pte(1, PTE_R))); // R set but V not set
        assert!(!is_valid(0));
    }

    #[test]
    fn test_is_leaf() {
        assert!(is_leaf(make_pte(1, PTE_V | PTE_R)));
        assert!(is_leaf(make_pte(1, PTE_V | PTE_X)));
        assert!(is_leaf(make_pte(1, PTE_V | PTE_R | PTE_W | PTE_X)));
        // 非叶节点：仅 V 被置位，R/W/X 都未置位
        assert!(!is_leaf(make_pte(1, PTE_V)));
        assert!(!is_leaf(make_pte(1, PTE_V | PTE_A | PTE_D)));
    }

    #[test]
    fn test_check_permission_read() {
        let pte = make_pte(1, PTE_V | PTE_R);
        assert!(check_permission(pte, true, false, false));
        assert!(!check_permission(pte, false, true, false));
        assert!(!check_permission(pte, false, false, true));
    }

    #[test]
    fn test_check_permission_rw() {
        let pte = make_pte(1, PTE_V | PTE_R | PTE_W);
        assert!(check_permission(pte, true, true, false));
        assert!(!check_permission(pte, true, true, true));
    }

    #[test]
    fn test_check_permission_all() {
        let pte = make_pte(1, PTE_V | PTE_R | PTE_W | PTE_X);
        assert!(check_permission(pte, true, true, true));
        assert!(check_permission(pte, true, false, false));
        assert!(check_permission(pte, false, false, false)); // no requirement = OK
    }

    #[test]
    fn test_check_permission_invalid() {
        // V 未置位，即使有 R/W/X 标志也应返回 false
        let pte = make_pte(1, PTE_R | PTE_W | PTE_X);
        assert!(!check_permission(pte, true, false, false));
    }
}

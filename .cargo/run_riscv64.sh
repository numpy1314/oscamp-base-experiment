#!/usr/bin/env bash
# Cargo runner for riscv64gc-unknown-linux-gnu：
# 本机为 riscv64 时直接执行；否则用 qemu-riscv64 用户态运行。
# 可通过环境变量 RISCV64_SYSROOT 指定根文件系统（默认 /usr/riscv64-linux-gnu）。

set -e
if [ "$(uname -m)" = "riscv64" ]; then
    exec "$@"
else
    SYSROOT="${RISCV64_SYSROOT:-/usr/riscv64-linux-gnu}"
    if ! command -v qemu-riscv64 >/dev/null 2>&1; then
        echo "错误: 未找到 qemu-riscv64。请安装 QEMU 用户态，例如：" >&2
        echo "  Debian/Ubuntu: sudo apt-get install qemu-user-static" >&2
        echo "  Fedora: sudo dnf install qemu-user-static" >&2
        exit 1
    fi
    if [ ! -d "$SYSROOT" ]; then
        echo "错误: 未找到 riscv64 根文件系统 $SYSROOT。请安装，例如：" >&2
        echo "  Debian/Ubuntu: sudo apt-get install gcc-riscv64-linux-gnu" >&2
        echo "  或设置环境变量: export RISCV64_SYSROOT=/path/to/sysroot" >&2
        exit 1
    fi
    exec qemu-riscv64 -L "$SYSROOT" "$@"
fi

# NekoOS Structure Notes

## Repository Layout

- `kernel/` — kernel crate (boot, arch, memory, traps, tasks, devices)
- `ulib/` — minimal user-space support library (provides `_start`, expects `fn main() -> i32`)
- `docs/` — design notes and developer docs
- `Makefile`, `env.mk` — build, run, and environment checks
- `Cargo.toml`, `rust-toolchain.toml` — workspace and toolchain configuration

## Kernel Layout

- `arch/` — architecture-specific code (RISC-V)
- `algorithm/` — allocators and schedulers
- `dev/` — device drivers (console, timer, cpu, fdt)
- `mm/` — memory management (heap, page allocator, page tables, VM space)
- `task/` — process and thread management
- `trap/` — trap and syscall handling

## Boot Flow (riscv64)

1. `kernel/src/arch/riscv64/entry.asm` sets up the boot stack, enables paging (Sv39),
   and jumps to `start`.
2. `kernel/src/main.rs:start` initializes console, memory, timer, traps, and task
   structures, then arms the timer and exits via `dev::cpu::shutdown()`.

## Build Outputs

- `target/<target>/<mode>/kernel` — ELF image (for debugging)
- `target/<target>/<mode>/neko-kernel.bin` — raw binary (for QEMU)

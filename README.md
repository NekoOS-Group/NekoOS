<div align="center">
  <a href="https://github.com/NekoOS-group/NekoOS">
    <img src="docs/image/Neko.jpeg" alt="Logo" width="130" height="100">
  </a>

  <h1 align="center">NekoOS</h1>
  <p align="center">
    A Rust-based RISC-V operating system kernel
    <br />
    <a href="https://github.com/NekoOS-group/NekoOS"><strong>Explore the docs »</strong></a>
  </p>

  <p align="center">
    <a href="https://github.com/NekoOS-group/NekoOS/graphs/contributors">
      <img src="https://img.shields.io/github/contributors/NekoOS-group/NekoOS.svg?style=for-the-badge" alt="Contributors">
    </a>
    <a href="https://github.com/NekoOS-group/NekoOS/network/members">
      <img src="https://img.shields.io/github/forks/NekoOS-group/NekoOS.svg?style=for-the-badge" alt="Forks">
    </a>
    <a href="https://github.com/NekoOS-group/NekoOS/stargazers">
      <img src="https://img.shields.io/github/stars/NekoOS-group/NekoOS.svg?style=for-the-badge" alt="Stars">
    </a>
    <a href="https://github.com/NekoOS-group/NekoOS/issues">
      <img src="https://img.shields.io/github/issues/NekoOS-group/NekoOS.svg?style=for-the-badge" alt="Issues">
    </a>
    <a href="https://github.com/NekoOS-group/NekoOS/blob/master/LICENSE">
      <img src="https://img.shields.io/github/license/NekoOS-group/NekoOS.svg?style=for-the-badge" alt="License">
    </a>
  </p>

  <p align="center">
    <a href="https://github.com/NekoOS-group/NekoOS">View Demo</a>
    ·
    <a href="https://github.com/NekoOS-group/NekoOS/issues">Report Bug</a>
    ·
    <a href="https://github.com/NekoOS-group/NekoOS/issues">Request Feature</a>
  </p>
</div>

## About The Project

NekoOS is a Rust-based RISC-V operating system kernel focused on a small, readable codebase for learning and experimentation. It is `no_std`, targets bare metal, and runs on QEMU.

Built from the ground up using Rust's safety features, NekoOS provides a playground for OS development enthusiasts and a practical example of systems programming in Rust.

## Project Structure

- `kernel/` — kernel crate (arch, mm, trap, task, dev, algorithms)
- `ulib/` — minimal user-space support library
- `docs/` — design notes and developer docs
- `Makefile`, `env.mk` — build, run, and environment checks
- `Cargo.toml` — workspace definition

## Getting Started

### Prerequisites

To build and run NekoOS, you'll need:

1. **Rust Toolchain:**
   ```bash
   # Install rustup if you haven't already
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Install the stable toolchain
   rustup toolchain install stable

   # Target and components
   rustup target add riscv64gc-unknown-none-elf
   rustup component add rust-src llvm-tools-preview
   cargo install cargo-binutils
   ```

2. **QEMU:**
   - Ubuntu/Debian: `sudo apt install qemu-system`
   - macOS: `brew install qemu`

3. **Debugging tools** (optional):
   - GDB: `sudo apt install gdb-multiarch` (Ubuntu/Debian)
   - LLDB: Available with most Rust installations

### Building and Running

1. **Verify your environment:**
   ```bash
   make check-env
   ```

2. **Build the kernel:**
   ```bash
   make build
   ```

3. **Run the kernel in QEMU:**
   ```bash
   make run LOG=INFO
   ```
   `LOG` is compiled into the kernel. Change the value and rebuild to update log output.

4. **Debug the kernel:**
   ```bash
   make debug     # Start QEMU in debug mode
   make gdb       # Connect with GDB
   # or
   make lldb      # Connect with LLDB
   ```

## Docs

- `docs/STRUCTURE.md` — kernel layout and boot flow notes

## Todo List

- [x] debug/backtrace
- [ ] dev/block
- [ ] dev/char
- [x] dev/console
- [ ] dev/cpu
- [x] dev/fdt
- [x] dev/timer
- [ ] fs
- [x] mm/kernel heap (todo: oom handler)
- [x] mm/page table
- [x] mm/riscv64 (sv39; todo: sv48, sv57, sv64)
- [ ] mm/riscv32 (sv32)
- [ ] syscall
- [ ] schedule (todo: restructure schedule into algorithm)
- [ ] schedule/scheduler
- [ ] schedule/task/abi
- [ ] schedule/task/process
- [ ] schedule/task/thread
- [ ] schedule/task/idle
- [x] trap (arch independent)
- [ ] ulib

## 🤝 Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions to NekoOS are **greatly appreciated**.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/amazing-feature`)
3. Commit your Changes (`git commit -m 'Add some amazing feature'`)
4. Push to the Branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please make sure to follow our coding standards and test your changes before submitting.

## License

Distributed under the GPL-3.0 License. See `LICENSE` for more information.

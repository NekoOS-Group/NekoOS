<br />
<div align="center">
  <a href="https://github.com/othneildrew/Best-README-Template">
    <img src="doc/image/Neko.jpeg" alt="Logo" width="130" height="100">
  </a>

  <h3 align="center">NekoOS-Kernel</h3>
</div>

-----

<div align="center">
  <p align="center">
    A rust based risc-v operating system!
    <br />
    <a href="https://github.com/NekoOS-group/NekoOS-kernel"><strong>Explore the docs »</strong></a>
  </p>
</div>


<div align="center">
  <a href="https://github.com/NekoOS-group/NekoOS-kernel/graphs/contributors">
  <img src="https://img.shields.io/github/contributors/NekoOS-group/NekoOS-kernel.svg?style=for-the-badge"></img></a>
  <a href="https://github.com/NekoOS-group/NekoOS-kernel/network/members">
  <img src="https://img.shields.io/github/forks/NekoOS-group/NekoOS-kernel.svg?style=for-the-badge"></img></a>
  <a href="https://github.com/NekoOS-group/NekoOS-kernel/stargazers">
  <img src="https://img.shields.io/github/stars/NekoOS-group/NekoOS-kernel.svg?style=for-the-badge"></img></a>
  <a href="https://github.com/NekoOS-group/NekoOS-kernel/issues">
  <img src="https://img.shields.io/github/issues/NekoOS-group/NekoOS-kernel.svg?style=for-the-badge"></img></a>
  <a href="https://github.com/NekoOS-group/NekoOS-kernel/blob/master/LICENSE.txt">
  <img src="https://img.shields.io/github/license/NekoOS-group/NekoOS-kernel.svg?style=for-the-badge"></img>
  </a>
</div>

<div align="center">
  <p align="center">
    <br />
    <a href="https://github.com/NekoOS-group/NekoOS-kernel">View Demo</a>
    ·
    <a href="https://github.com/NekoOS-group/NekoOS-kernel/issues">Report Bug</a>
    ·
    <a href="https://github.com/NekoOS-group/NekoOS-kernel/issues">Request Feature</a>
  </p>
</div>

## About The Project

Neko is kawaii!

## Getting Start

### Prerequisites

This is an example on Ubuntu to install the requirement.

 - rust environment
   ```
   sudo apt install rustup
   rustup install nightly
   rustup target add riscv64gc-unknown-none-elf
   cargo install cargo-binutils
   rustup component add rust-src
   rustup component add llvm-tools-preview
   ```

 - qemu  
   ```
   sudo apt install qemu
   ```

### Compile and Run

you can run with.
```
make run LOG=DEBUG
```

## Todo List
  - [ ] kernel
    - [ ] dev
    - [ ] fs
    - [ ] mm
      - [ ] memory detect
      - [ ] memory segment manager
      - [x] page allocator
      - [ ] page table 
    - [ ] syscall
    - [ ] task
    - [x] trap
  - [ ] ulib

## Contribute

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## License

Distributed under the GPL-3.0 License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Contact

bzy - [@bzyawa](https://twitter.com/bzyawa) - bzy.cirno@gmail.com

<p align="right">(<a href="#readme-top">back to top</a>)</p>
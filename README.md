# My rCore Learning

Follow the tutorial [rCore-tutorial-Book](https://rcore-os.cn/rCore-Tutorial-Book-v3/index.html), and I'm trying to finish every practices after each chapter.

I record most of my gains in the README in practice folder for each chapter, and some others are record on my [blog](https://zooeywm.github.io/blog/)

Hoping we can learn together!

## Current work tree

```
├── bootloader # The SBI with M-privilege which the core runs on
│   └── rustsbi-qemu.bin # RustSBI pre-compiled qemu version
├── os # Core implement
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src # Core source code
│   │   ├── entry.asm # Assembly code for setting core environment
│   │   ├── lang_items.rs # Semantic items provide to rustc
│   │   ├── linker.ld # Linker for core running on qemu
│   │   └── main.rs # Core main function
├── practice # Practices for each chapter
│   └── ch0
├── README.md
└── rust-toolchain.toml # Tool chain control
```

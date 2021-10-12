# Requirements
```
$ rustup component add rust-src llvm-tools-preview
$ cargo install bootimage
```

# Build
```
$ cargo bootimage
```

# Run on QEMU
```
$ qemu-system-x86_64 -drive format=raw,file=target/x86_64-oos/debug/bootimage-oos.bin
```

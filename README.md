# Build

```
# Allow cargo to access the rust source code,
# in order to compile standard libs described in .cargo/config.toml.
$ rustup component add rust-src

$ cargo build --target x86_64-oos.json
```

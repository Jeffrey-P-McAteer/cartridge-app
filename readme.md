
# Cartridge app

A simple application which enables the core feature of Cartridge OS
on an existing system (linux and windows 10 supported)

# Compiling on Linux

```bash
# Builds target/x86_64-pc-windows-gnu/release/cartridge_app.exe
cross build --release --target x86_64-pc-windows-gnu
# Builds target/x86_64-unknown-linux-gnu/release/cartridge_app
cargo build --release --target x86_64-unknown-linux-gnu
```


# Cartridge app

A simple application which enables the core feature of Cartridge OS
on an existing system (linux and windows 10 supported)

# Built binaries

Linux and windows x64 binaries are available at https://www.cs.odu.edu/~jmcateer/cartridge_app/

# Compiling on Linux

```bash
# Builds target/x86_64-pc-windows-gnu/release/cartridge_app.exe
#cross build --release --target x86_64-pc-windows-gnu
# ^^ Cross compilation is borked after we started using SDL2

# Builds target/x86_64-unknown-linux-gnu/release/cartridge_app
cargo build --release --target x86_64-unknown-linux-gnu
```

# Compiling on windows


```bash
# Builds target/x86_64-pc-windows-gnu/release/cartridge_app.exe
cargo build --release --target x86_64-pc-windows-gnu

```

# Historical Notice

This project has gone through a lot of thrashing about messing with different
styles of cross-platform compilation, GUI libraries, and even wandering into the
abyss that is the win32 API.

As a result this project is nowhere near idiomatic Rust code, and is very poorly designed.

The majority of the code written has been removed, re-written, and then replaced again.



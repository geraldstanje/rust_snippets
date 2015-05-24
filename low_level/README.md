# toycompiler

## Goal:

This project shows how to cross compile your Rust application for ARM devices.

## Installation:

To try the release you'll need:

* Rust Cross Compiler for ARM:
  $ git clone https://github.com/rust-lang/rust
  $ cd rust
  $ git checkout stable
  $ ./configure --target=arm-unknown-linux-gnueabihf,x86_64-unknown-linux-gnu
  $ make -j$(nproc)
  $ sudo make install

* Cargo:
  curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --prefix=local_cargo

* Reference:
  https://github.com/japaric/ruststrap/blob/master/1-how-to-cross-compile.md

## Cross compiling with Cargo:

$ cargo build --target=arm-unknown-linux-gnueabihf
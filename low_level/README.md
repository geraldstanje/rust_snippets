## Goal:

This project shows how to cross compile your Rust application for ARM devices.

## Installation:

* Install the Rust Cross Compiler (Rust 1.0.0) for ARM:
```
  $ git clone https://github.com/rust-lang/rust
  $ cd rust
  $ git checkout stable
  $ ./configure --target=arm-unknown-linux-gnueabihf,x86_64-unknown-linux-gnu
  $ make -j$(nproc)
  $ sudo make install
```

* Build Cargo:
```
  $ mkdir local_cargo
  $ curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --prefix=local_cargo
```

* Reference:
  https://github.com/japaric/ruststrap/blob/master/1-how-to-cross-compile.md

## Cross compile your Rust App with Cargo:
```
$ cargo build --target=arm-unknown-linux-gnueabihf
```

## Run the deploy script:
Set the Cargo Path and the IP address of your ARM device to RP_IP.

```
$ ./deploy
```
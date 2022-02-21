This repo illustrate the how to compile rust program with ASAN that works in hybrid scenario.

## Requirements

* LLVM-10 (installed from package manager, not tested on self compiled version yet)

* Unstable rust

```bash
rustup default nightly
```

## Usage

Compile library (gnu make required, you may also read the `Makefile` and compile it manually)

```bash
cd lib
make
```

Build rust program

```bash
RUSTFLAGS="-Z sanitizer=address" cargo build
```

Run

```bash
LD_LIBRARY_PATH=./lib ./target/debug/program 1  # array declared in rust but accessed in library
LD_LIBRARY_PATH=./lib ./target/debug/program 2  # array declared in library and accessed in library
LD_LIBRARY_PATH=./lib ./target/debug/program 3 # array declared in library but accessed in rust

```
## Requirements
1. **rust**
2. **gcc**
3. **cbindgen**

## Compile
#### Rust library
```shell
$ cd testlib
$ cbindgen --crate testlib --output ../testlib.h --lang c
$ cargo build --release
```
#### C executable
```shell
$ cp testlib/target/release/libtestlib.so libtestlib.so
$ gcc example.c -L. -ltestlib -o example.out
$ chmod +x example.out
```

## Execute
```shell
$ export LD_LIBRARY_PATH=.; ./example.out
```
(Otherwise, the library can be loaded at runtime using the `dlfcn.h` library. This repository does not offer an example for this alternative.)

## Useful links
1. [A little Rust with your C](https://rust-embedded.github.io/book/interoperability/rust-with-c.html)
2. [Minimizing Rust Binary Size](https://github.com/johnthagen/min-sized-rust#minimizing-rust-binary-size)
3. [CBindGen Quick Start](https://github.com/eqrion/cbindgen#quick-start)
4. [Creating a shared and static library with gcc](https://renenyffenegger.ch/notes/development/languages/C-C-plus-plus/GCC/create-libraries/index)

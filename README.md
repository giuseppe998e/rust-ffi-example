## Requirements
1. **rust** (with *cargo*)
2. **gcc**
3. **cbindgen**

## Compile the Rust library
```shell
$ cd testlib
$ cargo build --release
$ cbindgen --crate testlib --lang c --output ../testlib.h
$ cp target/release/libtestlib.{so,a} ..
```

## Compile the C executable
#### Using the dynamic lib
```shell
$ gcc example.c -ltestlib -o example.out
```
#### Using the static lib
```shell
$ gcc example.c libtestlib.a -ldl -pthread -o example.out
```

## Execute
```shell
$ chmod +x example.out
$ LD_LIBRARY_PATH=. ./example.out
```
N.B. The environment variable `LD_LIBRARY_PATH` is only needed for the dynamic version of the library.

---

The Rust library could also be loaded at runtime using the `dlfcn.h` library.  
This repository does not offer an example for this alternative.

## Useful links
1. [FFI - Wikipedia](https://en.wikipedia.org/wiki/Foreign_function_interface)
2. [FFI - Rustonomicon](https://doc.rust-lang.org/nomicon/ffi.html)
2. [A little Rust with your C](https://rust-embedded.github.io/book/interoperability/rust-with-c.html)
3. [Exposing FFI from the Rust library](https://svartalf.info/posts/2019-03-01-exposing-ffi-from-the-rust-library/)
4. [Creating a shared and static library with gcc](https://renenyffenegger.ch/notes/development/languages/C-C-plus-plus/GCC/create-libraries/index)
5. [Minimizing Rust Binary Size](https://github.com/johnthagen/min-sized-rust#minimizing-rust-binary-size)
6. [CBindGen Quick Start](https://github.com/eqrion/cbindgen#quick-start)

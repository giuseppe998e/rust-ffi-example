# Rust library linked to a C program
An example of how to create a dynamic library (.so) created with the Rust language and link it to a program written in C language.

--
### Requirements
1. rust
2. gcc
3. cbindgen

### Compile
#### Rust lib
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

### Execute
```shell
$ export LD_LIBRARY_PATH=.; ./example.out
```

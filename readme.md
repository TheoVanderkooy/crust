
C <-> rust cross-compilation POC

This example consumes both the rust code from C _and_ C code from rust, not just one direction.

The general idea is:
 1. Compile all C sources to object files, with no linking yet.
 2. Rust code is organized as a static library, and passes the C object files to the linker.
 3. Then use `gcc` to link everything back in with the main function, including the static rust library.

If the main entry-point is in rust, the last step is unnecessary, with just a binary crate.

An alternate approach would be to use rustc directly with `--emit=obj --crate-type=staticlib` args to generate object files, and set up the makefile in the same way as for C. But this makes it more work to migrate to cargo later.





Interesting notes about C<->rust FFI in general
===============================================


Bindgen
-------
Running manually to generate bindings from a header file:
```sh
bindgen include/foo.h -o foo-rs/src/temp_bindngs.rs \
    --use-core
```



Strings
-------
Rust-allocated strings are _not_ safe to free in C, they have to be passed back to be deallocated!
 - [https://jakegoulding.com/rust-ffi-omnibus/string_return/]

C <-> rust cross-compilation POC

This example consumes both the rust code from C _and_ C code from rust, not just one direction.

The general idea is:
 1. Compile all C sources to object files, with no linking yet.
 2. Rust code is organized as a static library, and passes the C object files to the linker.
 3. Then use `gcc` to link everything back in with the main function, including the static rust library.

If the main entry-point is in rust, the last step is unnecessary, with just a binary crate.

An alternate approach would be to use rustc directly with `--emit=obj --crate-type=staticlib` args to generate object files, and set up the makefile in the same way as for C. But this makes it more work to migrate to cargo later.
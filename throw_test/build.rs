

extern crate cc;


fn main() {
    cc::Build::new().file("src/test.c").compile("test");
    // println!("rustc-link-lib=static=test.o");
}
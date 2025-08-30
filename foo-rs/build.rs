

const OBJECT_FILES: &[&'static str] = &[
    "foo2",
];


fn main() {
    // use std::{env, path::PathBuf, str::FromStr};
    // let Ok(mut src_dir) = PathBuf::from_str(&env::var("CARGO_MANIFEST_DIR").unwrap());
    // src_dir.push("../src");
    // let lib_dir = src_dir.to_string_lossy();

    let lib_dir = "../src";

    for &p in OBJECT_FILES {
        // updating any object files needs to re-link with rust
        println!("cargo::rerun-if-changed={lib_dir}/{p}.o", );

        // all C object files need to be passed to the linker
        println!("cargo::rustc-link-arg={lib_dir}/{p}.o");
    }

    // println!("cargo:warning=>>>testing<<<");
}

// println!("cargo:rustc-link-search=native={}", lib_dir);
// println!("cargo:rustc-link-lib=static=foo2");
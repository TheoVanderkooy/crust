use pgrx::{PgTryBuilder, pg_extern, pg_guard};
use pgrx;






// Using rust-analyzer: Expand maro recursively at caret

fn main() {
    println!("Hello, world!");
}


// Converts rust panic to ereport, and ereports to panic (why?)
#[pg_guard]
extern "C-unwind" fn test() -> i32 {
    // ...
    let x = 3;
    let y = x + 1;
//
unsafe { pgrx::pg_sys::submodules::panic::pgrx_extern_c_guard(||{ /* function body */ }) };
    y
}


#[pg_guard]
extern "C-unwind" {
    fn test_unimplemented() -> i32;
}

/*
pgrx_extern_c_guard:

...

*/

// does nothing??
#[pg_guard]
extern "C-unwind" {
    fn test1() -> u64 { 32 }
}

// doesn't work
// #[pg_guard] fn test2() -> u8 { 2}


// lets you use the function as a UDF in rust. way more complicated that what we want just for C calling rust code
#[pg_extern]
fn test3() {
    PgTryBuilder::new(|| {
        // ...
    });
}


fn test4() {
    #[pg_guard]
    test();
}
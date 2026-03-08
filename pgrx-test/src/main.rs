use pgrx::{PgTryBuilder, pg_extern, pg_guard};








fn main() {
    println!("Hello, world!");
}


// Converts rust panic to ereport, and ereports to panic (why?)
#[pg_guard]
extern "C-unwind" fn test() -> i32 {
    // ...
    let x = 3;
    let y = x + 1;
unsage { pgrx::pg_sys::submodules::panic::pgrx_extern_c_guard(||{}) };
    y
}

// does nothing??
#[pg_guard]
extern "C-unwind" {
    fn test1() -> u64 { 32 }
}

// doesn't work
// #[pg_guard] fn test2() -> u8 { 2}


// lets you use the function as a UDF in rust
#[pg_extern]
fn test3() {
    PgTryBuilder::new(|| {
        // ...
    });
}
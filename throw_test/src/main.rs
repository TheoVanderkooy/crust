#[allow(warnings)]
#[allow(unused)]
#[allow(unused_imports)]

mod bindings;

use std::ffi::c_int;
use std::mem::MaybeUninit;

use crate::bindings::{__sigsetjmp, PG_exception_stack, sigjmp_buf};
// use crate::bindings::throws; // this is imported by the macro instead

use c_import::{c_import, rust_export};



#[c_import(throws)]
fn wrap_throws() ;


#[c_import(maybe_throws)]
fn wrap_maybe_throws(x: c_int, dothrow: bool) -> c_int;


#[rust_export(crwap_call_rust_fn)]
pub fn export_to_c(x: c_int) -> Result<c_int, PgError> {
    Ok(2 * x)
}


pub struct PgError;

/// Proof of concept of catching a C longjmp "exception".
/// NOTE: `sigsetjmp` is actually a macro in C, so this is not portable. Should use https://lib.rs/crates/setjmp, or a C shim instead
fn catches_from_c() -> Result<(), PgError> {
    unsafe {
        let save_stack = PG_exception_stack;
        let mut local_jmp_buf: sigjmp_buf = MaybeUninit::zeroed().assume_init();
        if __sigsetjmp(local_jmp_buf.as_mut_ptr(), 1) == 0 {
            PG_exception_stack = &mut local_jmp_buf;

            // println!("{:?}", {PG_exception_stack});

            // `try` body
            {
                throws();
                println!("RUST: didn't throw!");
            }

        } else {
            PG_exception_stack = save_stack;

            println!("RUST: caught something! returning error");
            return Err(PgError);
        }
        PG_exception_stack = save_stack;

    }

    Ok(())
}


// TODO: for rust code called from C, provide a way to re-throw
// TODO: make sure we don't forget that we got an exception from C.. maybe provide a way to clear the error

fn main() {
    println!("Hello, world!");

// pgrx uses a C shim for longjmp:
//   https://github.com/pgcentralfoundation/pgrx/blob/87db532f1d473663faf34333d7311bca424ddd77/pgrx-pg-sys/pgrx-cshim.c#L29
//   https://github.com/pgcentralfoundation/pgrx/blob/87db532f1d473663faf34333d7311bca424ddd77/pgrx-pg-sys/src/submodules/ffi.rs#L29
// a few other libraries for setjmp/jongjmp:
//   https://lib.rs/crates/setjmp
//   https://github.com/jordanisaacs/sjlj

    match catches_from_c() {
        Ok(()) => println!("catches_from_c() succeeded"),
        Err(_) => println!("catches_from_c() got an error",),
    }

    match unsafe { wrap_throws() } {
        Ok(()) => println!("catches_from_c() succeeded"),
        Err(_) => println!("catches_from_c() got an error",),
    }


    match unsafe { wrap_maybe_throws(3, false) } {
        Ok(ret) => println!("Ran maybe_throws and got result: {ret}"),
        Err(_) => println!("maybe_throws(false) threw! ERROR"),
    }

    match unsafe { wrap_maybe_throws(5, true) } {
        Ok(ret) => println!("Ran maybe_throws and got result: {ret} ERROR"),
        Err(_) => println!("maybe_throws(true) threw!"),
    }

}

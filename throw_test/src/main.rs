#[allow(warnings)]
#[allow(unused)]
#[allow(unused_imports)]
mod bindings;

use std::{ffi::c_int, mem::transmute};
use std::mem::MaybeUninit;

use cee_scape::call_with_sigsetjmp;
use setjmp::{jmp_buf, sigsetjmp};

use crate::bindings::PG_exception_stack;
// use crate::bindings::throws; // this is imported by the macro instead

use c_import::{c_import, c_import_infallible, rust_export};

unsafe trait PgArg: Sized + Copy {}
unsafe impl PgArg for c_int {}

unsafe trait PgRet: Sized + Copy {}
unsafe impl PgRet for c_int {}
unsafe impl PgRet for () {}


#[c_import(throws)]
fn wrap_throws();

#[c_import(maybe_throws)]
fn wrap_maybe_throws(x: c_int, dothrow: bool) -> c_int;

#[c_import]
fn maybe_throws(x: c_int, dothrow: bool) -> c_int;

#[c_import_infallible(maybe_throws)]
fn wrap_maybe_throws_2(x: c_int, dothrow: bool) -> c_int;

#[rust_export(cwrap_fn_no_result)]
pub fn export_to_c_2(x: c_int) -> c_int {
    3 * x
}

#[rust_export(cwrap_fn_no_ret)]
pub fn export_to_c_3() {
    println!("no return")
}

#[rust_export(crwap_call_rust_fn)]
pub fn export_to_c(x: c_int) -> Result<c_int, PgError> {
    Ok(2 * x)
}

pub enum PgError {
    PgPassthrough, // indicates something still in the PG error stack
    New {
        // ...
        // level
        // line
        // function
        // file
        // errmsg
        // errhint
        // ...
        // errcode
        // backtrace
        // ...
    }
}

/// Proof of concept of catching a C longjmp "exception".
/// NOTE: `sigsetjmp` is actually a macro in C, so this is not portable. Should use https://lib.rs/crates/setjmp, or a C shim instead
fn catches_from_c() -> Result<(), PgError> {
    unsafe {
        let save_stack = PG_exception_stack;
        let mut local_jmp_buf: MaybeUninit<jmp_buf> = MaybeUninit::uninit();
        if sigsetjmp(local_jmp_buf.as_mut_ptr(), 1) == 0 {
            PG_exception_stack = transmute(local_jmp_buf.as_mut_ptr());

            // println!("{:?}", {PG_exception_stack});

            // `try` body
            {
                use crate::bindings::throws;
                throws();
                println!("RUST: didn't throw!");
            }
        } else {
            PG_exception_stack = save_stack;

            println!("RUST: caught something! returning error");
            return Err(PgError::PgPassthrough);
        }
        PG_exception_stack = save_stack;
    }

    Ok(())
}


fn catches_from_c_cscape() -> Result<(), PgError> {
    unsafe {
        let save_stack = PG_exception_stack;
        if call_with_sigsetjmp(true, |env| {
            PG_exception_stack = env;
            use crate::bindings::throws;
            throws();
            0
        }) != 0 {
            println!("RUST: cee-scape caugth something! returning error");
            PG_exception_stack = save_stack;
            return Err(PgError::PgPassthrough);
        }
        PG_exception_stack = save_stack;
        Ok(())
    }
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

    // docs: https://en.cppreference.com/w/cpp/utility/program/setjmp

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

    match unsafe { maybe_throws(5, true) } {
        Ok(ret) => println!("Ran maybe_throws and got result: {ret} ERROR"),
        Err(_) => println!("maybe_throws(true) threw!"),
    }

    unsafe { wrap_maybe_throws_2(5, false)} ;

    // println!("next thing should panic:");
    // unsafe { wrap_maybe_throws_2(5, true)} ;

    match catches_from_c_cscape() {
        Ok(()) => println!("catches_from_c_cscape succeeded"),
        Err(_) => println!("catches_from_c_cscape got an error"),
    }


    // sanity check of cee_scape longjmp
    unsafe {
        assert_eq!(5, call_with_sigsetjmp(true, |env| {
            cee_scape::siglongjmp(env, 5);
        }));
    }

}

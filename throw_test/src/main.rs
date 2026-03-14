


mod bindings;

use std::mem::MaybeUninit;

use crate::bindings::{__sigsetjmp, PG_exception_stack, sigjmp_buf};
use crate::bindings::throws;

// unsafe extern "C-unwind" {
//     unsafe fn  throws();
// }


struct PgError;

// TODO: macro-ize this
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
}

use crate::{TEST_GLOBAL_ALLOC_COUNT, TEST_GLOBAL_DEALLOC_COUNT};
use core::ffi::{CStr, c_char, c_int};
use std::ffi::CString;

// Imports from C-provided functions
unsafe extern "C" {
    fn test2(x_: c_int) -> c_int;
}

// Exported to C
#[unsafe(no_mangle)]
pub extern "C" fn test(str_: *mut c_char) -> c_int {
    let msg = unsafe { CStr::from_ptr(str_) }.to_string_lossy();
    println!("got message in rust: {msg}");
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!(
        "Global alloc count = {}, dealloc = {}",
        unsafe { TEST_GLOBAL_ALLOC_COUNT },
        unsafe { TEST_GLOBAL_DEALLOC_COUNT }
    );
    unsafe { test2(12 + v.len() as i32) }
}

// Exported to C
#[unsafe(no_mangle)]
pub extern "C" fn test_str(x: c_int) -> *const c_char {
    let ret = {
        let msg: String = format!("string generated in rust, with input {x}");
        CString::new(msg).unwrap().into_raw()
    };

    println!(
        "Global alloc count = {}, dealloc = {}",
        unsafe { TEST_GLOBAL_ALLOC_COUNT },
        unsafe { TEST_GLOBAL_DEALLOC_COUNT }
    );
    ret
}

use core::ffi::{c_char, c_int, CStr};

unsafe extern "C" {
    fn test2(x_: c_int) -> c_int;
}

#[unsafe(no_mangle)]
pub extern "C" fn test(str_: *mut c_char) -> c_int {
    let msg = unsafe { CStr::from_ptr(str_) }.to_string_lossy();
    println!("got message in rust: {msg}");
    unsafe { test2(12) }
}

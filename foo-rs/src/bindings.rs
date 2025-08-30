use std::ffi::CStr;

unsafe extern "C" {
    fn test2(x_: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}

#[unsafe(no_mangle)]
pub extern "C" fn test(str_: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int {
    let msg = unsafe { CStr::from_ptr(str_) }.to_string_lossy();
    println!("got message in rust: {msg}");
    unsafe { test2(12) }
}

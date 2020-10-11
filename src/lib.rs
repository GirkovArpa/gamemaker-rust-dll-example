// $ cargo +stable-i686-pc-windows-gnu build --release
extern crate libc;
use libc::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use std::i64;

#[no_mangle]
pub extern "cdecl" fn foo(bar: f64) -> () {
    println!("bar: {:?}", bar);
}

#[no_mangle]
pub extern "cdecl" fn bar(baz: *const c_char) -> () {
    let c_str: &CStr = unsafe { CStr::from_ptr(baz) };
    let str_slice: &str = c_str.to_str().unwrap();
    println!("baz: {}", str_slice);
}   

#[no_mangle]
pub extern "cdecl" fn print_window_handle(handle: *const c_char) -> () {
    let c_str: &CStr = unsafe { CStr::from_ptr(handle) };
    let hex_string: &str = c_str.to_str().unwrap();
    let handle = i64::from_str_radix(hex_string, 16);
    println!("HWND -  HEX: {}, DEC: {:?}", hex_string, handle);
}

#[no_mangle]
pub extern "cdecl" fn foo_bar_baz() -> *const c_char {
    let c_string = CString::new("foo bar baz").unwrap();
    let pointer = c_string.into_raw();
    pointer
}

#[no_mangle]
pub extern "cdecl" fn elite_number() -> f64 {
    1337.0f64
}
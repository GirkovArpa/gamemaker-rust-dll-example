# GameMaker / Rust Demos

Sending and recieving numbers, strings, and window handles.

Tested with GameMaker Runtime Version **2.2.2.326**.

In some examples, some dependencies may be omitted to brevity, but they can be seen in [src/lib.rs](src/lib.rs).

In `Cargo.toml`:
```toml
[lib]
crate-type = ["cdylib"]
```

## Sending number (ty_real):

```rust
// src/lib.rs
#[no_mangle]
pub extern "cdecl" fn foo(bar: f64) -> () {
    println!("{:?}", bar);
}
```
Compile as 32bit DLL for compatibility with the 32bit YoYoGames runner:

```
$ cargo +stable-i686-pc-windows-gnu build --release
```

GameMaker script:

```gml
var foo = external_define("gamemaker_rust_dll_example.dll", "foo", dll_cdecl, ty_real, 1, ty_real);
external_call(foo, 123);
```

## Sending string (ty_string):

In `Cargo.toml`:
```toml
[dependencies]
libc ="*"
```

```rust
extern crate libc;
use libc::c_char;
use std::ffi::CStr;

#[no_mangle]
pub extern "cdecl" fn bar(baz: *const c_char) -> () {
    let c_str: &CStr = unsafe { CStr::from_ptr(baz) };
    let str_slice: &str = c_str.to_str().unwrap();
    println!("baz: {}", str_slice);
}   
```

```gml
var bar = external_define("gamemaker_rust_dll_example.dll", "bar", dll_cdecl, ty_real, 1, ty_string);
external_call(bar, "baz");
```

## Sending window_handle()

In GameMaker, the result of `window_handle()` must be cast into a (hex) `ty_string` with `string()`.

```gml
var print_window_handle = external_define("gamemaker_rust_dll_example.dll", "print_window_handle", dll_cdecl, ty_real, 1, ty_string);
external_call(print_window_handle, string(window_handle()));
```

In Rust, the hex string must be interpreted as a `HWND`, which is basically an `i64`.  Although there is some nuance here that you can google for, such as the value will never exceed the maximum value of an `i32`, or something along those lines.

```rust
use std::i64;

// ...

#[no_mangle]
pub extern "cdecl" fn print_window_handle(handle: *const c_char) -> () {
    let c_str: &CStr = unsafe { CStr::from_ptr(handle) };
    let hex_string: &str = c_str.to_str().unwrap();
    let handle = i64::from_str_radix(hex_string, 16);
    println!("HWND -  HEX: {}, DEC: {:?}", hex_string, handle);
}
```

## Recieving string

```gml
var foo_bar_baz = external_define(
    "gamemaker_rust_dll_example.dll", 
    "foo_bar_baz", 
    dll_cdecl, 
    ty_string, 
    0
);
show_debug_message(external_call(foo_bar_baz)); // => foo bar baz
```

```rust
#[no_mangle]
pub extern "cdecl" fn foo_bar_baz() -> *const c_char {
    let c_string = CString::new("foo bar baz").unwrap();
    let pointer = c_string.into_raw();
    pointer
}
```

## Recieving number

```gml
var elite_number = external_define("gamemaker_rust_dll_example.dll", "elite_number", dll_cdecl, ty_real, 0);
show_debug_message(external_call(elite_number)); // => 1337
```

```rust
#[no_mangle]
pub extern "cdecl" fn elite_number() -> f64 {
    1337.0f64
}
```
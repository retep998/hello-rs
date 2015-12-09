#![deny(warnings)]
#![feature(no_core, lang_items, intrinsics, start)]
#![no_core]

#[macro_use]
extern crate syscall;

#[no_mangle]
pub extern fn _start() {
    unsafe {
        syscall!(WRITE, 1, b"Hello, world!\n" as *const _, 14);
        syscall!(EXIT, 0);
    }
}

#[lang = "eh_personality"] extern fn eh_personality() { }
#[start] fn start(_argc: isize, _argv: *const *const u8) -> isize { 0 }

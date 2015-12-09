#![deny(warnings)]
#![feature(no_core, lang_items, intrinsics)]
#![no_core]
#![crate_type = "lib"] // So we don't need a start function :P

extern "rust-intrinsic" {
    fn transmute<T, U>(t: T) -> U;
}

const CALL: [u8; 29] = [0x48, 0x89, 0xfe, 0xbf, 0x01, 0x00, 0x00, 0x00, 0xba,
    0x0e, 0x00, 0x00, 0x00, 0xb8, 0x01, 0x00, 0x00, 0x00, 0x0f, 0x05, 0xb8,
    0x3c, 0x00, 0x00, 0x00, 0x31, 0xff, 0x0f, 0x05];

#[no_mangle]
pub extern fn _start() {
    let call: extern fn(&[u8; 14]) -> ! = unsafe { transmute(&CALL) };
    call(b"Hello, world!\n");
}

#[lang = "sized"] trait Sized { }
#[lang = "copy"] trait Copy { }

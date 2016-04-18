#![feature(lang_items)]
#![no_std]
#![no_main]

// Executables must define the `eh_personality` and `panic_fmt` lang items.
// Normally libstd does this for us; however we only have libcore, and we don't
// need or want the defaults anyway, since our code literally can't panic.

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}

// kernel32 FFI bindings.
enum CVoid {}

type BOOL    = i32;
type DWORD   = u32;
type LPDWORD = *mut DWORD;
type HANDLE  = *mut CVoid;
type VOID    = CVoid;
type LPVOID  = *mut VOID;

const STD_OUTPUT_HANDLE: DWORD = 0xfffffff4;

#[link(name = "kernel32")]
extern "system" {
    fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
    fn WriteConsoleA(
        hConsoleOutput:         HANDLE,
        lpBuffer:               *const VOID,
        nNumberOfCharsToWrite:  DWORD,
        lpNumberOfCharsWritten: LPDWORD,
        lpReserved:             LPVOID
    ) -> BOOL;
}

// Program entry point. Windows directly calls into this function - no shimming
// shenanigans here!
#[no_mangle] pub extern "system" fn entry_point() {
    use core::ptr;
    let msg = "Hello world!\n";
    let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };
    unsafe { WriteConsoleA(stdout, msg.as_ptr() as *const VOID, msg.len() as DWORD, ptr::null_mut(), ptr::null_mut()) };
}

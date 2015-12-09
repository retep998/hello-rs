extern crate winapi;
extern crate kernel32;
use std::ptr;
fn main() {
    let msg = "Hello world!\n";
    let stdout = unsafe { kernel32::GetStdHandle(winapi::STD_OUTPUT_HANDLE) };
    unsafe { kernel32::WriteConsoleA(stdout, msg.as_ptr() as *const winapi::VOID, msg.len() as winapi::DWORD, ptr::null_mut(), ptr::null_mut()) };
}
#[link(name = "ucrt")]
#[link(name = "vcruntime")]
extern {}
#[no_mangle] pub extern "system" fn entry_point() { main() }

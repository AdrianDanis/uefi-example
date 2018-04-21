#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![no_std]
#![no_main]

extern crate rlibc;
extern crate compiler_builtins;

extern crate uefi;

use uefi::{Handle, Status};
use uefi::table;

#[macro_use]
mod print;
mod panic;

pub use panic::rust_begin_panic;

pub static mut UEFI_SYSTEM_TABLE: Option<&'static table::SystemTable> = None;

#[no_mangle]
pub extern "win64" fn UefiMain(_handle: Handle, st: &'static table::SystemTable) -> Status {
    unsafe {
        UEFI_SYSTEM_TABLE = Some(&st);
    }
    loader_main();
    Status::Success
}

fn loader_main() -> () {
    println!("Hello world!");
    unimplemented!();
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    unimplemented!();
}


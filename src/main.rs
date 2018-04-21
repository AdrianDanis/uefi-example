#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![feature(duration_extras)]
#![no_std]
#![no_main]

extern crate rlibc;
extern crate compiler_builtins;

extern crate uefi;
extern crate fencedvar;

use uefi::{Handle, Status};
use uefi::table;

#[macro_use]
mod print;
mod panic;
mod intrinsics;

pub use panic::rust_begin_panic;
pub use intrinsics::*;

pub static mut UEFI_SYSTEM_TABLE: Option<&'static table::SystemTable> = None;

#[no_mangle]
pub extern "win64" fn UefiMain(_handle: Handle, st: &'static table::SystemTable) -> Status {
    unsafe {
        UEFI_SYSTEM_TABLE = Some(&st);
    }
    panic::set_panic_action(panic::Action::ShutdownDelay(core::time::Duration::from_secs(1)));
    loader_main();
    Status::Success
}

fn loader_main() -> () {
    println!("Hello world!");
    unimplemented!();
}


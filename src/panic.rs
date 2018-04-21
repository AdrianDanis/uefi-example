use core;
use uefi;

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments,
                               file: &'static str,
                               line: u32,
                               column: u32) -> ! {
    println!("Panic at {} {}:{}: {}", file, line, column, msg);
    println!("Powering down in 5 seconds...");
    let boottime = unsafe {::UEFI_SYSTEM_TABLE.unwrap().boot};
    boottime.stall(5000000);
    let runtime = unsafe {::UEFI_SYSTEM_TABLE.unwrap().runtime};
    runtime.reset(uefi::table::runtime::ResetType::Shutdown, uefi::Status::Aborted, None)
}

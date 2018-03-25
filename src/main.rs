#![feature(lang_items)]
#![feature(compiler_builtins_lib)]
#![no_std]
#![no_main]

extern crate rlibc;
extern crate compiler_builtins;

extern crate uefi;

use core::fmt;

use uefi::{Handle, Status};
use uefi::table;

pub static mut UEFI_SYSTEM_TABLE: Option<&'static table::SystemTable> = None;

macro_rules! print {
    ($($arg:tt)*) => (::write_console_fmt(format_args!($($arg)*)).unwrap());
}

macro_rules! println {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

struct ConsoleWriter(&'static mut uefi::proto::console::text::Output);

impl fmt::Write for ConsoleWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Allocate a small buffer on the stack.
        const BUF_SIZE: usize = 128;
        // Add 1 extra character for the null terminator.
        let mut buf = [0u16; BUF_SIZE + 1];

        let mut i = 0;

        // This closure writes the local buffer to the output and resets the buffer.
        let mut flush_buffer = |buf: &mut [u16], i: &mut usize| {
            buf[*i] = 0;
            *i = 0;

            self.0.output_string(buf.as_ptr()).map_err(
                |_| fmt::Error,
            )
        };

        {
            // This closure converts a character to UCS-2 and adds it to the buffer,
            // flushing it as necessary.
            let mut add_char = |ch| {
                // UEFI only supports UCS-2 characters, not UTF-16,
                // so there are no multibyte characters.
                let ch = ch as u16;

                buf[i] = ch;

                i += 1;

                if i == BUF_SIZE {
                    flush_buffer(&mut buf, &mut i)
                } else {
                    Ok(())
                }
            };

            for ch in s.chars() {
                if ch == '\n' {
                    // Prepend an '\r'.
                    add_char('\r')?;
                }

                add_char(ch)?;
            }
        }

        // Flush whatever is left in the buffer.
        flush_buffer(&mut buf, &mut i)
    }
}

fn write_console_fmt(args: fmt::Arguments) -> fmt::Result {
    let output = unsafe {UEFI_SYSTEM_TABLE.unwrap().stdout()};
    let mut cw = ConsoleWriter(output);
    fmt::Write::write_fmt(&mut cw, args)
}

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

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn rust_begin_panic(msg: core::fmt::Arguments,
                               file: &'static str,
                               line: u32,
                               column: u32) -> ! {
    println!("Panic at {} {}:{}: {}", file, line, column, msg);
    println!("Powering down in 5 seconds...");
    let boottime = unsafe {UEFI_SYSTEM_TABLE.unwrap().boot};
    boottime.stall(5000000);
    let runtime = unsafe {UEFI_SYSTEM_TABLE.unwrap().runtime};
    runtime.reset(table::runtime::ResetType::Shutdown, Status::Aborted, None)
}

#[no_mangle]
pub extern "C" fn __floatundisf() {
    unimplemented!();
}


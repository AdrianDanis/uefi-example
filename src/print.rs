use core::fmt;
use uefi;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::write_console_fmt(format_args!($($arg)*)).unwrap());
}

#[macro_export]
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

pub fn write_console_fmt(args: fmt::Arguments) -> fmt::Result {
    let output = unsafe {::UEFI_SYSTEM_TABLE.unwrap().stdout()};
    let mut cw = ConsoleWriter(output);
    fmt::Write::write_fmt(&mut cw, args)
}

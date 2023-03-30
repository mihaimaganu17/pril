//! Module that holds the print macro

// Dummy writer we can implement `Write` trait on, so that we can support formatted strings
pub struct ConsoleOutWriter;

impl core::fmt::Write for ConsoleOutWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Calls our own UEFI implementation for outputting a string to the console
        crate::efi::uefi_print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ( $($arg:tt)* ) => {
        let mut console_writer = $crate::print::ConsoleOutWriter;
        core::fmt::write(&mut console_writer, core::format_args!($($arg)*)).unwrap();
    }
}

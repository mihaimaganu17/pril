#![feature(panic_info_message)]
#![no_std]
#![no_main]

pub mod efi;
mod panic;

use crate::efi::{initialize_system_table, EfiHandle, EfiStatus, EfiSystemTable};

use core::fmt::{self, Write};
use core::format_args;

// Dummy writer we can implement `Write` trait on, so that we can support formatted strings
struct ConsoleOutWriter;

impl fmt::Write for ConsoleOutWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Calls our own UEFI implementation for outputting a string to the console
        efi::uefi_print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ( $($arg:tt)* ) => {
        let mut console_writer = ConsoleOutWriter;
        write!(console_writer, "{}", $crate::format_args!($($arg)*)).unwrap();
    }
}

#[no_mangle]
extern "C" fn efi_main(_image_handle: EfiHandle, system_table: *mut EfiSystemTable) -> EfiStatus {
    initialize_system_table(system_table);
    print!("{}", 2 + 2);
    panic!("Prea Mult Gogosi, dar nu e niciodata prea tarziu sa mai pui niste gogosi");
}

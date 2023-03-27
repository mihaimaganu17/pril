#![feature(panic_info_message)]
#![no_std]
#![no_main]

mod panic;
pub mod efi;

use core::sync::atomic::Ordering;
use crate::efi::{EfiSystemTable, EfiHandle, EfiStatus, EFI_SYSTEM_TABLE, EfiSimpleTextOutputProtocol, initialize_system_table};

// TODO: Write function to get system table easily without dereferencing it every time(maybe)
// TODO: Refactor print logic in it own module
// TODO: Document EFI structures that we need
// TODO: Refactor out_str in another file

// Takes a `str` slice as input and displays it in the default UEFI ConsoleOut device
fn out_str(to_print: &str) {
    // Load the EFI system table
    let sys_table = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    // If there is a null pointer, we cannot print
    if sys_table.is_null() {
        return;
    }

    let text_out_protocol = unsafe {
        (*sys_table).con_out as *const EfiSimpleTextOutputProtocol
    };

    // Declare a temporary buffer that we will use to output the string to the console
    let mut tmp: [u16; 32] = [0; 32];

    let mut tmp_idx: usize = 0;
    for utf16_chr in to_print.encode_utf16() {
        // Copy the character into the buffer
        tmp[tmp_idx] = utf16_chr;
        tmp_idx += 1;

        // If out buffer is full, output it to the display
        if tmp_idx == tmp.len() - 1{
            // Append a null at the end of the buffer
            tmp[tmp_idx] = 0;
            unsafe {
                ((*text_out_protocol).output_string)(text_out_protocol, tmp.as_ptr());
            }
            tmp_idx = 0;
        }
    }

    // If after finishing iterating through the slice, we still have characters in the buffer,
    // we just print them
    if tmp_idx != 0 {
        // Append a null at the end of the buffer
        tmp[tmp_idx] = 0;
        unsafe {
            ((*text_out_protocol).output_string)(text_out_protocol, tmp.as_ptr());
        }
    }
}

use core::format_args;
use core::fmt::{self, Write};

// Dummy writer we can implement `Write` trait on, so that we can support formatted strings
struct ConsoleOutWriter;

impl fmt::Write for ConsoleOutWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Calls our own UEFI implementation for outputting a string to the console
        out_str(s);
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

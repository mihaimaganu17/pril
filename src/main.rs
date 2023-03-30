#![feature(panic_info_message)]
#![no_std]
#![no_main]

pub mod efi;
pub mod print;
mod panic;

use crate::efi::{get_memory_map, initialize_system_table, EfiHandle, EfiStatus, EfiSystemTable};

#[no_mangle]
extern "C" fn efi_main(_image_handle: EfiHandle, system_table: *mut EfiSystemTable) -> EfiStatus {
    initialize_system_table(system_table);
    get_memory_map();
    print!("{}", 2 + 2);
    panic!("Prea Mult Gogosi, dar nu e niciodata prea tarziu sa mai pui niste gogosi");
}

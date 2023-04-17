#![feature(panic_info_message)]
#![feature(strict_provenance)]
#![no_std]
#![no_main]

pub mod efi;
pub mod print;
mod panic;

use crate::efi::{get_memory_map, initialize_system_table, EfiHandle, EfiStatus, EfiSystemTable};

#[no_mangle]
extern "C" fn efi_main(_image_handle: EfiHandle, system_table: *mut EfiSystemTable) -> EfiStatus {
    initialize_system_table(system_table);

    let map_key = get_memory_map();

    assert!(map_key != 0);

    crate::efi::read_config_table();

    print!("This is a formatted string {:?}!!!\n", 2+2);

    //exit_boot_services(image_handle, map_key);
    panic!("Pril finished running\n");
}

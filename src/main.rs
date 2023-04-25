#![feature(panic_info_message)]
#![feature(strict_provenance)]
#![no_std]
#![no_main]

pub mod efi;
mod panic;
pub mod print;

use crate::efi::{initialize_system_table, EfiHandle, EfiStatus, EfiSystemTable};
use crate::efi::malloc::EfiMemoryManager;

#[no_mangle]
extern "C" fn efi_main(_image_handle: EfiHandle, system_table: *mut EfiSystemTable) -> EfiStatus {
    initialize_system_table(system_table);

    let mut mem_manager =  EfiMemoryManager::new();
    let map_key = mem_manager.get_memory_map();

    let total_avlbl_mem = mem_manager.free_mem_after_exit_bs();

    print!("Total available memory {}!!!\n", total_avlbl_mem);

    loop {}
}

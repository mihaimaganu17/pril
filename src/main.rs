#![feature(panic_info_message)]
#![feature(strict_provenance)]
#![no_std]
#![no_main]

pub mod efi;
mod panic;
pub mod print;
pub(crate) mod cpu; 

use crate::efi::{initialize_system_table, EfiHandle, EfiStatus, EfiSystemTable};
use crate::efi::malloc::EfiMemoryManager;
use cpu::msr_reg_addr;

#[no_mangle]
extern "C" fn efi_main(_image_handle: EfiHandle, system_table: *mut EfiSystemTable) -> EfiStatus {
    initialize_system_table(system_table);

    let mut mem_manager =  EfiMemoryManager::new();
    let map_key = mem_manager.get_memory_map();

    let total_avlbl_mem = mem_manager.free_mem_after_exit_bs();

    print!("Total available memory {}!!!\n", total_avlbl_mem);

    let cr0 = unsafe { cpu::cr0() };
    let ia_efer = unsafe { cpu::rdmsr(msr_reg_addr::IA32_EFER) };

    print!("Cr0: {:#?}\n", cr0);
    print!("ia_efer: {:#b}\n", ia_efer);


    loop {}
}

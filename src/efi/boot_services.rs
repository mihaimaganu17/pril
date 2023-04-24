//! Module that handles all of the EFI Boot Services table functions
use crate::{
    efi::{status, EfiTableHeader, EFI_SYSTEM_TABLE},
    EfiHandle, EfiStatus,
};
use core::sync::atomic::Ordering;

/// Signature for the `EfiBootServicesTable` structure
pub const EFI_BOOT_SERVICES_SIGNATURE: u64 = 0x5652_4553_544f_4f42;

/// Represents the EFI Boot Service Table, which contains a table header and pointers to all of the
/// boot services as described in the Boot Service chapter from any UEFI Spec.
/// The function pointers in this table are not valied after the OS has taken control of the
/// platform with a call to `exit_boot_services`.
#[repr(C)]
pub struct EfiBootServicesTable {
    /// Header for this table
    pub hdr: EfiTableHeader,
    // From here, until the end of this structure, all the fields represent function pointers
    //
    // Task priority services, from EFI 1.0+
    //
    _raise_tpl: usize,
    _restore_tpl: usize,
    //
    // Memory Services, all from EFI 1.0+
    //
    _allocate_pages: usize,
    _free_pages: usize,
    // Returns the current boot services memory map and memory map key
    pub get_memory_map: fn(
        memory_map_size: &mut usize,
        memory_map: *mut u8,
        map_key: &mut usize,
        descriptor_size: &mut usize,
        descriptor_version: &mut u32,
    ) -> EfiStatus,
    _allocate_pool: usize,
    _free_pool: usize,
    //
    // Event & Timer Services, all from EFI 1.0+
    //
    _create_event: usize,
    _set_timer: usize,
    _wait_for_event: usize,
    _signal_event: usize,
    _close_event: usize,
    _check_event: usize,
    //
    // Protocol Handler Services, all from EFI 1.0+
    //
    _install_protocol_interface: usize,
    _reinstall_protocol_interface: usize,
    _uninstall_protocol_interface: usize,
    _handle_protocol: usize,
    _reserved: usize,
    _register_protocol_notify: usize,
    _locate_handle: usize,
    _locate_device_path: usize,
    _install_configuration_table: usize,
    //
    // Image Services, all from EFI 1.0+
    //
    _image_load: usize,
    _image_start: usize,
    _exit: usize,
    _image_unload: usize,
    /// Terminates boot services
    exit_boot_services: fn(image_handle: EfiHandle, map_key: usize) -> EfiStatus,
    //
    // Miscellaneous Services, all from EFI 1.0+
    //
    _get_next_monotonic_count: usize,
    _stall: usize,
    _set_watchdog_timer: usize,
    //
    // DriverSupport Services
    //
    _connect_controller: usize,
    _disconnect_controller: usize,
    //
    // Open and Close Protocol Services, all from EFI 1.1+
    //
    _open_protocol: usize,
    _close_protocol: usize,
    _open_protocol_information: usize,
    //
    // Library Services, all from EFI 1.1+
    //
    _protocols_per_handle: usize,
    _locate_handle_buffer: usize,
    _locate_protocol: usize,
    _install_multiple_protocol_interfaces: usize,
    _uninstall_multiple_protocol_interfaces: usize,
    //
    // 32-bit CRC Services, all from EFI 1.1+
    //
    _calculate_crc32: usize,
    //
    // Miscellaneous services
    //
    // from EFI 1.1+
    _copy_mem: usize,
    // from EFI 1.1+
    _set_mem: usize,
    // from UEFI 2.0+
    _create_event_ex: usize,
}

pub fn exit_boot_services(image_handle: EfiHandle, map_key: usize) {
    // Get a hold of the global EFI System Table
    let sys_table = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    // Check if it is a valid pointer
    if sys_table.is_null() {
        return;
    }

    // Get a reference to the boot services table
    let boot_services_table = unsafe { (*sys_table).boot_services };

    let status = unsafe { ((*boot_services_table).exit_boot_services)(image_handle, map_key) };
    assert!(status == status::EFI_SUCCESS);

    EFI_SYSTEM_TABLE.store(core::ptr::null_mut(), Ordering::SeqCst);
}

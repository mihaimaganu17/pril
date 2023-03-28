//! Module that handles all of the EFI Boot Services table functions
use bitflags::bitflags;
use crate::{EfiStatus, efi::EfiTableHeader};

/// Signature for the `EfiBootServicesTable` structure
pub const EFI_BOOT_SERVICES_SIGNATURE: u64 = 0x5652_4553_544f_4f42;

/// Represents the EFI Boot Service Table, which contains a table header and pointers to all of the
/// boot services as described in the Boot Service chapter from any UEFI Spec.
/// The function pointers in this table are not valied after the OS has taken control of the
/// platform with a call to `exit_boot_services`.
#[derive(Debug)]
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
    get_memory_map: usize,
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
    exit_boot_services: usize,
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

impl EfiBootServicesTable {
    /// Returns the current boot services memory map and memory map key where
    /// - `memory_map_size` is a pointer to the size, in bytes, of the `memory_map` buffer.
    /// On input, this is the size of the buffer allocated by the caller.
    /// On output, it is the size of the buffer returned by the firmware if the buffer was large
    /// enough, or the size of the buffer needed to contain the map if the buffer was too small.
    /// - `memory_map` is a pointer to the buffer in which firmware places the current memory map.
    /// The map is an array of `EfiMemoryDescriptor`s
    /// - `map_key` is a pointer to the location in which firmware returns the key for the current
    /// memory map.
    /// - `descriptor_size` is a pointer to the location in which firmware returns the size, in
    /// bytes, of an individual `EfiMemoryDescriptor`.
    /// - `descriptor_version` is a pointer to the location in which firmware returns the version
    /// number associated with the `EfiMemoryDescriptor`.
    pub fn get_memory_map(
        memory_map_size: usize,
        memory_map: &mut [u8],
        map_key: usize,
        descriptor_size: usize,
        descriptor_version: usize,
    ) -> EfiStatus {
        0
    }
}

// Type that represents a UEFI Physical Address
type EfiPhysicalAddress = u64;
// Type that represents a UEFI Virtual Address
type EfiVirtualAddress = u64;

// Memory descriptor version number
const EFI_MEMORY_DESCRIPTOR_VERSION: u8 = 1;

/// Structure that describes a single memory map entry from `EfiBootServicesTable` memory map
#[derive(Debug)]
#[repr(C)]
pub struct EfiMemoryDescriptor {
    // Type of the memory region
    mem_type: EfiMemoryType,
    // Physical start regions, which must be aligned on a 4KiB boundary and must not be
    // above 0xffff_ffff_ffff_f000
    phys_start: EfiPhysicalAddress,
    // Virtual address of the frist byte in the memory region. It must be 4KiB aligned and must not
    // be above 0xffff_ffff_ffff_f000.
    virt_start: EfiVirtualAddress,
    // Number of 4KiB pages in the memory region. This number MUST NOT be 0 and must not be
    // any value that would represent a memory page with a start address, either physical or
    // virtual, above 0xffff_ffff_ffff_f000.
    number_pages: u64,
    // Attributes of the memory region that describe the bit mask capabilities for that memory
    // region and not necessarily the current settings for that memory region.
    attr_mask: EfiMemoryAttributes,
}

#[derive(Debug)]
pub enum EfiMemoryType {
}

bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct EfiMemoryAttributes: u64 {
        //
        // Memory cacheability attribues
        //
        // The memory region supports being configured as not cacheable.
        const UC = 0x0000_0000_0000_0001;
        // The memory region supports being configured as write combining
        const WC = 0x0000_0000_0000_0002;
        // The memory region supports being configured as cacheable with a "write through" policy.
        // Writes that hit in the cache will also be written to main memory.
        const WT = 0x0000_0000_0000_0004;
        // The memory region supports being configured as cacheable, with a "write back" policy.
        // Reads and writes that hit in the cache do not propagate to main memory. Dirty data is
        // written back to main memory when a new cache line is allocated.
        const WB = 0x0000_0000_0000_0008;
        // The memory region supports being configured as not cacheable, exported, and supports
        // the "fetch and add" semaphore mechanism.
        const UCE = 0x0000_0000_0000_0010;

        //
        // Physical memory protection attributes
        //
        // The memory regions supports being configures as write-protected by system hardware.
        // This is typically used as a cacheability attributes today. The memory region supports
        // being configured as cacheable with a `write protected` policy. Reads come from cache
        // lines when possible, and read misses cause cache fills. Writes are propagated to the
        // system bus and cause corresponding cache lines on all processors on the bus to be
        // invalidated.
        const WP = 0x0000_0000_0000_1000;
        // The memory region supports being configured as read-protected by system hardware.
        const RP = 0x0000_0000_0000_2000;
        // The memory region supports being configured so it is protected by system hardware from
        // executing code.
        const XP = 0x0000_0000_0000_4000;
        // The memory region supports making this memory range read-only by system hardware
        const RO = 0x0000_0000_0002_0000;

        //
        // Runtime memory attributes
        //
        // The memory region refers to persistent memory
        const NV = 0x0000_0000_0000_8000;
        // The memory region need to be given a virtual mapping by the operating system when
        // `SetVirtualAddressMap()` is called
        const RUNTIME = 0x8000_0000_0000_0000;

        // The memory region provides higher reliability relative to other memory in the system.
        // If all memory has the same reliability, then this bit is not used.
        const MORE_RELIABLE = 0x0000_0000_0001_0000;
    }
}



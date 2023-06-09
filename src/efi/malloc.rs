//! Module that describes the functions that make up Memory Allocation Services in UEFI and are
//! used during preboot to allocate and free memory, and to obtain the system’s memory map.
//!
//! By UEFI's default memory design, EFI firmware owns the system's memory map during preboot.
//! Key features:
//! 1. During preboot, all components(includeing executing EFI images) must cooperate with the
//! firmware by allocating and freeing memory from the system using the EFI's incorporated memory
//! allocation functions.
//! 2. During preboot, an executing EFI Image must only use the memory is has allocated.
//! 3. Before an executing EFI image exits and returns control to the firmware, it must free
//! all resources it has explicitly allocated. This includes all memory pages, pool allocations,
//! open file handles, etc. Memory allocated by the firmware to load an image is freed by the
//! firmware when the image is unloaded.
use crate::{
    efi::{status, EFI_SYSTEM_TABLE}, print,
};
use bitflags::bitflags;
use core::sync::atomic::Ordering;

/// This is the size of the buffer we use to retrieve the memory map from UEFI
pub const MEMORY_MAP_BUFFER_SIZE: usize = 8 * 1024;

/// This is the maximum number of entries we can have reported by the EFI GetMemoryMap
pub const MAX_MEMORY_MAP_ENTRIES: usize = 1000;

/// This is the defaul UEFI page size
pub const EFI_PAGE_SIZE: usize = 4 * 1024;

pub struct EfiMemoryManager {
    memory_pool: [Option<EfiMemoryDescriptor>; MAX_MEMORY_MAP_ENTRIES],
}

// We make a constant that is able to initialize a memory pool that is larger than 32 elements
const INIT_MEMORY_POOL: Option<EfiMemoryDescriptor> = None;

impl EfiMemoryManager {
    pub fn new() -> Self {
        Self {
            memory_pool: [INIT_MEMORY_POOL; MAX_MEMORY_MAP_ENTRIES],
        }
    }

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
    ///
    /// This function returns the map key obtained from a `get_memory_map` call
    pub fn get_memory_map(&mut self) -> usize {
        // Get a hold of the global EFI System Table
        let sys_table = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

        // Check if it is a valid pointer
        if sys_table.is_null() {
            return 0;
        }

        // Get a reference to the boot services table
        let boot_services_table = unsafe { (*sys_table).boot_services };
        let mut memory_map_size: usize = MEMORY_MAP_BUFFER_SIZE;
        let mut memory_map = [0u8; MEMORY_MAP_BUFFER_SIZE];
        let mut map_key: usize = 0;
        let mut descriptor_size: usize = 0;
        let mut descriptor_version: u32 = 0;

        let status = unsafe {
            ((*boot_services_table).get_memory_map)(
                &mut memory_map_size,
                memory_map.as_mut_ptr(),
                &mut map_key,
                &mut descriptor_size,
                &mut descriptor_version,
            )
        };

        // Printing affects the memory map, and the map key will change. However, if our status
        // is not successful we want to know about it
        match status {
            status::EFI_BUFFER_TOO_SMALL => {
                // The memory map buffer was too small
                print!(
                    "Memory map size is too small! Retrying with size: {}\n",
                    memory_map_size
                );
                // If we cannot obtain a memory map the second time, just panic
                if status != status::EFI_SUCCESS {
                    panic!("Cannot get the memory map, even with the right size\n");
                }
            }
            status::EFI_INVALID_PARAMETER => {
                // The memory_map buffer is NULL. This should be impossible and we will panic if this
                // is the case
                panic!("Memory map buffer is NULL\n");
            }
            status::EFI_SUCCESS => {
                //print!("Successfully got a memory map!\n");
            }
            _ => {
                panic!("Unknown get memory map status code {}", status);
            }
        };

        for (idx, offset) in (0..memory_map_size).step_by(descriptor_size).enumerate() {
            let entry = unsafe {
                core::ptr::read_unaligned(
                    memory_map.get(offset..).unwrap().as_ptr() as *const EfiMemoryDescriptor
                )
            };

            self.memory_pool[idx] = Some(entry);
        }

        map_key
    }

    /// Reports the free memory after exiting the boot services
    pub fn free_mem_after_exit_bs(&self) -> u64 {
        // Initialize the total available memory
        let mut total_avlbl_mem = 0;
        for maybe_entry in &self.memory_pool {
            if let Some(entry) = maybe_entry {
                match entry.mem_type {
                    EfiMemoryType::BootServicesCode
                    | EfiMemoryType::BootServicesData
                    | EfiMemoryType::ConventionalMemory
                    | EfiMemoryType::PersistentMemory
                    | EfiMemoryType::LoaderCode
                    | EfiMemoryType::LoaderData => {
                        // Compute the total available memory
                        total_avlbl_mem += entry.number_pages * EFI_PAGE_SIZE as u64;
                    } 
                    _ => {}
                }
            }
        }

        total_avlbl_mem
    }
}




// Type that represents a UEFI Physical Address
type EfiPhysicalAddress = u64;
// Type that represents a UEFI Virtual Address
type EfiVirtualAddress = u64;

// Memory descriptor version number
const _EFI_MEMORY_DESCRIPTOR_VERSION: u8 = 1;

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

impl From<u32> for EfiMemoryType {
    fn from(value: u32) -> Self {
        match value {
            0 => Self::ReservedMemoryType,
            1 => Self::LoaderCode,
            2 => Self::LoaderData,
            3 => Self::BootServicesCode,
            4 => Self::BootServicesData,
            5 => Self::RuntimeServicesCode,
            6 => Self::RuntimeServicesData,
            7 => Self::ConventionalMemory,
            8 => Self::UnusableMemory,
            9 => Self::ACPIReclaimMemory,
            10 => Self::ACPIMemoryNVS,
            11 => Self::MemoryMappedIO,
            12 => Self::MemoryMappedIOPortSpace,
            13 => Self::PalCode,
            14 => Self::PersistentMemory,
            _ => Self::MaxMemoryType,
        }
    }
}

/// Structure that describes the types of memory from the system, according to the UEFI Memory Map
/// Each memory type has one purpose BEFORE exiting Boot Services and another one after exiting
/// Boot Services
#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum EfiMemoryType {
    /// Before exiting Boot Sevices
    /// Not usable.
    /// After exiting Boot Services
    /// Not usable.
    ReservedMemoryType = 0,
    /// Before exiting Boot Sevices
    /// The code portions of a loaded UEFI application.
    /// After exiting Boot Services
    /// The UEFI OS Loader and/or OS may use this memory as they see fit. Note: the UEFI OS loader
    /// that called EFI_BOOT_SERVICES.ExitBootServices() is utilizing one or more EfiLoaderCode
    /// ranges.
    LoaderCode,
    /// Before exiting Boot Sevices
    /// The data portions of a loaded UEFI application and the default data allocation type used by
    /// a UEFI application to allocate pool memory.
    /// After exiting Boot Services
    /// The Loader and/or OS may use this memory as they see fit. Note: the OS loader that called
    /// ExitBootServices() is utilizing one or more EfiLoaderData ranges.
    LoaderData,
    /// Before exiting Boot Sevices
    /// The code portions of a loaded UEFI Boot Service Driver.
    /// After exiting Boot Services
    /// Memory available for general use.
    BootServicesCode,
    /// Before exiting Boot Sevices
    /// The data portions of a loaded UEFI Boot Serve Driver, and the default data allocation type
    /// used by a UEFI Boot Service Driver to allocate pool memory.
    /// After exiting Boot Services
    /// Memory available for general use.
    BootServicesData,
    /// Before exiting Boot Sevices
    /// The code portions of a loaded UEFI Runtime Driver.
    /// After exiting Boot Services
    /// The memory in this range is to be preserved by the UEFI OS loader and OS in the working and
    /// ACPI S1–S3 states.
    RuntimeServicesCode,
    /// Before exiting Boot Sevices
    /// The data portions of a loaded UEFI Runtime Driver and the default data allocation type used
    /// by a UEFI Runtime Driver to allocate pool memory.
    /// After exiting Boot Services
    /// The memory in this range is to be preserved by the UEFI OS l loader and OS in the working
    /// and ACPI S1–S3 states.
    RuntimeServicesData,
    /// Before exiting Boot Sevices
    /// Free (unallocated) memory.
    /// After exiting Boot Services
    /// Memory available for general use.
    ConventionalMemory,
    /// Before exiting Boot Sevices
    /// Memory in which errors have been detected.
    /// After exiting Boot Services
    /// Memory that contains errors and is not to be used.
    UnusableMemory,
    /// Before exiting Boot Sevices
    /// Memory that holds the ACPI tables.
    /// After exiting Boot Services
    /// This memory is to be preserved by the UEFI OS loader and OS until ACPI is enabled. Once
    /// ACPI is enabled, the memory in this range is available for general use.
    ACPIReclaimMemory,
    /// Before exiting Boot Sevices
    /// Address space reserved for use by the firmware.
    /// After exiting Boot Services
    /// This memory is to be preserved by the UEFI OS loader and OS in the working and ACPI S1–S3
    /// states.
    ACPIMemoryNVS,
    /// Before exiting Boot Sevices
    /// Used by system firmware to request that a memory-mapped IO region be mapped by the OS to a
    /// virtual address so it can be accessed by EFI runtime services.
    /// After exiting Boot Services
    /// This memory is not used by the OS. All system memory-mapped IO information should come from
    /// ACPI tables.
    MemoryMappedIO,
    /// Before exiting Boot Sevices
    /// System memory-mapped IO region that is used to translate memory cycles to IO cycles by the
    /// processor.
    /// After exiting Boot Services
    /// This memory is not used by the OS. All system memory-mapped IO port space information
    /// should come from ACPI tables.
    MemoryMappedIOPortSpace,
    /// Before exiting Boot Sevices
    /// Address space reserved by the firmware for code that is part of the processor.
    /// After exiting Boot Services
    /// This memory is to be preserved by the UEFI OS loader and OS in the working and ACPI S1–S4
    /// states. This memory may also have other attributes that are defined by the processor
    /// implementation.
    PalCode,
    /// Before exiting Boot Sevices
    /// A memory region that operates as EfiConventionalMemory. However, it happens to also support
    /// byte-addressable non-volatility.
    /// After exiting Boot Services
    /// Same as Before exiting
    PersistentMemory,
    /// Before exiting Boot Sevices
    MaxMemoryType,
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

        // Physical memory protection attribute: The memory region
        // supports making this memory range read-only by system
        // hardware.
        const MEMORY_RO = 0x0000000000020000;
        // Runtime memory attribute: The memory region needs to
        // be given a virtual mapping by the operating system when
        // SetVirtualAddressMap() is called
        const MEMORY_RUNTIME = 0x8000000000000000;
    }
}

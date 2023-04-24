//! Module that acts as a central point for FFI bindings from the UEFI API
pub mod acpi;
pub mod boot_services;
pub mod malloc;
pub mod status;

pub use boot_services::exit_boot_services;
pub use status::*;
pub use malloc::get_memory_map;
use acpi::EFI_ACPI_20_TABLE_GUID;
use boot_services::EfiBootServicesTable;
use core::sync::atomic::{AtomicPtr, Ordering};
use crate::print;

// Signature, that resides as the first field in the UEFI System Table. We check this to make sure
// we actually are in an UEFI system
const EFI_SYSTEM_TABLE_SIGNATURE: u64 = 0x5453_5953_2049_4249;

/// Pointer to the EFI System Table structure
pub static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(core::ptr::null_mut());

// This is only valid for x64 platforms, as each platform has a different handle type
pub type EfiHandle = usize;
// This is a handle to an event structure
type _EfiEvent = usize;
// Status code
pub type EfiStatus = usize;

/// Takes the `system_table` pointer given as input and places it into the global
/// `EFI_SYSTEM_TABLE`, if the global stores a null pointer.
pub fn initialize_system_table(system_table: *mut EfiSystemTable) {
    // Get the signature reported by UEFI system table
    let signature = unsafe { (*system_table).hdr.signature() };

    // Assert it is correct, just in case we are not passed a EfiSystemTable
    assert!(signature == EFI_SYSTEM_TABLE_SIGNATURE);

    // If the current pointer inside the `AtomicPtr` global is null, replace it with the passed
    // pointer
    EFI_SYSTEM_TABLE
        .compare_exchange(
            core::ptr::null_mut(),
            system_table,
            Ordering::SeqCst,
            Ordering::SeqCst,
        )
        .unwrap();
}

// Takes a `str` slice as input and displays it in the default UEFI ConsoleOut device
pub fn uefi_print(input: &str) {
    // Load the EFI System Table
    let sys_table = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    // If the System Table is a null-pointer, there is nothing we can do and we just return
    if sys_table.is_null() {
        return;
    }

    // Get the pointer to console out from the system table
    let text_out_protocol = unsafe { (*sys_table).con_out };

    // Declare a temporary buffer that we will use to output the string to the console
    let mut tmp: [u16; 32] = [0; 32];

    // Initialize an index which we will use to populate the temporary buffer
    let mut tmp_idx: usize = 0;

    // Go through each character in the given slice and encode it into utf16
    for utf16_chr in input.encode_utf16() {
        // Copy the character into the temporary buffer
        tmp[tmp_idx] = utf16_chr;
        // Go to the next free position in the buffer
        tmp_idx += 1;

        // If we reached the last position, our buffer is full and we output it to the display
        if tmp_idx == tmp.len() - 1 {
            // Append a null at the end of the buffer
            tmp[tmp_idx] = 0;
            // Send a pointer of the buffer to the console out `output_string` function
            unsafe {
                ((*text_out_protocol).output_string)(text_out_protocol, tmp.as_ptr());
            }
            // Reset to the first position
            tmp_idx = 0;
        }
    }

    // If after finishing iterating through the slice, we still have characters in the buffer,
    // we just print them
    if tmp_idx != 0 {
        // Append a null at the end of the buffer
        tmp[tmp_idx] = 0;

        // Send a pointer of the buffer to the console out `output_string` function
        unsafe {
            ((*text_out_protocol).output_string)(text_out_protocol, tmp.as_ptr());
        }
    }
}

/// Data structure that precedes all of the standard EFI table types.
#[derive(Debug)]
#[repr(C)]
pub struct EfiTableHeader {
    // A 64-bit signature that identifies the type of table that follows. Unique signatures have
    // been generated for the EfiSystemTable, the EfiBootServicesTable, and the
    // EfiRuntimeServicesTable.
    signature: u64,
    // The revision of the EFI Specification to which this table conforms. The upper 16 bits of
    // this field contain the major revision value, and the lower 16 bits contain the minor
    // revision value. The minor revision values are binary coded decimals and are limited to the
    // range of 00..99.
    revision: u32,
    // The size, in bytes, of the entire table including the `EfiTableHeader`.
    header_size: u32,
    // The 32-bit CRC for the entire table. This value is computed by setting this field to 0, and
    // computing the 32-bit CRC for `header_size` bytes
    // UEFI uses a standard CCITT32 CRC algorithm with a seed polynomial value of 0x04c11db7 for
    // its CRC calculations.
    crc32: u32,
    // Reserved field that must be set to 0.
    reserved: u32,
}

impl EfiTableHeader {
    pub fn signature(&self) -> u64 {
        self.signature
    }
    pub fn revision(&self) -> u32 {
        self.revision
    }
}

/// Contains pointers to the runtime and boot services tables.
/// Except for the table header `hdr`, all elements in the service tables are pointers to
/// functions.
/// After an operating system has taken control of the platform, using `exit_boot_services()`, from
/// the `boot_services` field, only the following fields remain valid.
/// - `hdr`
/// - `firmware_vendor`,
/// - `firmware_revision`,
/// - `runtime_services`,
/// - `number_of_table_entries`,
/// - `configuration_table`,
#[repr(C)]
pub struct EfiSystemTable {
    // The table header for this table. See `EfiTableHeader` for more info
    pub hdr: EfiTableHeader,
    // A pointer to a null terminated string that identifies the vendor that produces the system
    // firmare for the platform
    _firmware_vendor: usize,
    // A firmware vendor specific value that identifies the revision of the system firmware for the
    // platform.
    _firmware_revision: u32,
    // The handle for the active console input device. This handle must
    // support EFI_SIMPLE_TEXT_INPUT_PROTOCOL and EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.
    // See what those protocols are
    _console_in_handle: EfiHandle,
    // A pointer to the EFI_SIMPLE_TEXT_INPUT_PROTOCOL interface that is associated with
    // console_in_handle
    _con_in: usize,
    // The handle for the active console output device. This handle must support the
    // EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.
    console_out_handler: EfiHandle,
    /// A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is associated with
    /// ConsoleOutHandle.
    pub con_out: *const EfiSimpleTextOutputProtocol,
    // The handle for the active standard error console device. This handle must support the
    // EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL. If there is no active console, this protocol must still be
    // present.
    _std_err_handle: EfiHandle,
    // A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is associated with
    // StandardErrorHandle.
    _std_err: usize,
    // A pointer to the EFI Runtime Services Table.
    _runtime_services: usize,
    // A pointer to the EFI Boot Services Table.
    boot_services: *const EfiBootServicesTable,
    // The number of system configuration tables in the buffer ConfigurationTable.
    ntable_entries: usize,
    // A pointer to the system configuration tables. The number of entries in the table is
    // NumberOfTableEntries
    config_table: *const EfiConfigurationTableEntry,
}

// TODO: We should wrap these types intead so it is stronger typed
type EfiGuid = u128;

#[derive(Debug)]
#[repr(packed, C)]
pub struct EfiConfigurationTableEntry {
    // The 128-bit GUID value that uniquely identifies the system configuration table.
    pub vendor_guid: EfiGuid,
    // A pointer to the table associated with `vendor_guid`
    pub vendor_table: usize,
}

/// Reads the EfiConfigurationTable from the EfiSystemTable
pub fn read_config_table() {
    // Get a handle to the EfiSystemTable
    let sys_table = EFI_SYSTEM_TABLE.load(Ordering::SeqCst);

    // If the handle is null, just return
    if sys_table.is_null() {
        //print!("Is Null\n");
        return;
    }
    print!("Is Not Null!\n");

    // Compute the size of a single entry from the configuration table
    let config_table_entry_size = core::mem::size_of::<EfiConfigurationTableEntry>();

    // Get a handle to the configuration table
    let config_table = unsafe { (*sys_table).config_table };

    // Get the number of table entries
    let ntable_entries = unsafe { (*sys_table).ntable_entries };

    // We start at entry 0
    let mut entry_idx = 0;

    // Loop until we still have entries to parse
    while entry_idx < ntable_entries {
        // Compute the entry's address
        let entry_addr = config_table as usize + entry_idx * config_table_entry_size;
        // Read the entry and convert it to the appropriate structure
        let table_entry =
            unsafe { core::ptr::read_unaligned(entry_addr as *const EfiConfigurationTableEntry) };
        // Get the vendor guid
        let guid = table_entry.vendor_guid;

        if guid == EFI_ACPI_20_TABLE_GUID {
            crate::print!("Found ACPI 2.0 table: {:x?}\n", guid);
            acpi::read_rsdp(table_entry.vendor_table);
        }

        // Go to the next entry
        entry_idx += 1;
    }
}

/// The Simple Text Output Protocol defines the minimum requirements for a text-based `ConsoleOut`
/// device.
#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    // Reset the ConsoleOut device. EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.Reset().
    reset: usize,
    // Displays the string on the device at the current cursor location.
    // EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.OutputString() .
    pub output_string: unsafe fn(*const Self, *const u16) -> EfiStatus,
    // All the following fields are pointers to functions which we currently do not need
    _test_string: usize,
    _query_mode: usize,
    _set_mode: usize,
    _set_attribute: usize,
    _clear_screnn: usize,
    _set_cursor_pos: usize,
    _enable_cursor: usize,
    _mode: usize,
}

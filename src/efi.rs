//! Module that acts as a central point for FFI bindings from the UEFI API
use core::sync::atomic::{AtomicPtr, Ordering};

const EFI_SYSTEM_TABLE_SIGNATURE: u64 = 0x5453_5953_2049_4249;

/// Pointer to the EFI System Table structure
pub static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(core::ptr::null_mut());

// This is only valid for x64 platforms, as each platform has a different handle type
pub type EfiHandle = usize;
// This is a handle to an event structure
type _EfiEvent = usize;
// Status code
pub type EfiStatus = usize;

pub fn initialize_system_table(system_table: *mut EfiSystemTable) {
    // Get the signature reported by UEFI system table
    let signature = unsafe { (*system_table).hdr.signature() };

    // Assert it is correct, just in case we are not passed a EfiSystemTable
    assert!(signature == EFI_SYSTEM_TABLE_SIGNATURE);

    // If the current pointer inside the `AtomicPtr` global is null, replace it with the passed
    // pointer
    EFI_SYSTEM_TABLE.compare_exchange(
        core::ptr::null_mut(),
        system_table,
        Ordering::SeqCst,
        Ordering::SeqCst
    ).unwrap();
}

/// Data structu that precedes all of the standard EFI table types.
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
    // supportEFI_SIMPLE_TEXT_INPUT_PROTOCOL and EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.
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
    // A pointer to the EFI Boot Services Table. See
    // ref:efi-boot-services-table_efi_system_table.
    _boot_services: usize,
    // The number of system configuration tables in the buffer ConfigurationTable.
    _ntable_entries: usize,
    // A pointer to the system configuration tables. The number of entries in the table is
    // NumberOfTableEntries
    _config_table: usize,
}

/// The Simple Text Output protocol defines the minimum requirements for a text-based `ConsoleOut`
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

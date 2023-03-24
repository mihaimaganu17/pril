#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::arch::asm;
use core::sync::atomic::AtomicPtr;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// TODO: Read core::ptr
/// Data structu that precedes all of the standard EFI table types.
#[repr(C)]
pub struct EfiTableHeader {
    // A 64-bit signature that identifies the type of table that follows. Unique signatures have
    // been generated for the EfiSystemTable, the EfiBootServicesTable, and the
    // EfiRuntimeServicesTable.
    pub signature: u64,
    // The revision of the EFI Specification to which this table conforms. The upper 16 bits of
    // this field contain the major revision value, and the lower 16 bits contain the minor
    // revision value. The minor revision values are binary coded decimals and are limited to the
    // range of 00..99.
    pub revision: u32,
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

// This is only valid for x64 platforms, as each platform has a different handle type. Type Void *
type EfiHandle = usize;
// This is a handle to an event structure. Type Void *
type EfiEvent = usize;
// Status code
type EfiStatus = usize;

const EFI_SYSTEM_TABLE_SIGNATURE: u64 = 0x5453_5953_2049_4249;

#[repr(C)]
pub struct EfiSystemTable {
    // The table header for this table. See `EfiTableHeader` for more info
    pub hdr: EfiTableHeader,
    // A pointer to a null terminated string that identifies the vendor that produces the system
    // firmare for the platform
    firmware_vendor: usize,
    // A firmware vendor specific value that identifies the revision of the system firmware for the
    // platform.
    firmware_revision: u32,
    // The handle for the active console input device. This handle must
    // supportEFI_SIMPLE_TEXT_INPUT_PROTOCOL and EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL.
    // TODO: See what those protocols are
    console_in_handle: EfiHandle,
    // TODO: A pointer to the EFI_SIMPLE_TEXT_INPUT_PROTOCOL interface that is associated with
    // console_in_handle
    con_in: *const usize,
    // TODO: The handle for the active console output device. This handle must support the
    // EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.
    console_out_handler: EfiHandle,
    // TODO: A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is associated with
    // ConsoleOutHandle.
    con_out: usize,
    // TODO: The handle for the active standard error console device. This handle must support the
    // EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL. If there is no active console, this protocol must still be
    // present.
    std_err_handle: EfiHandle,
    // TODO: A pointer to the EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL interface that is associated with
    // StandardErrorHandle.
    std_err: *const usize,
    // TODO: A pointer to the EFI Runtime Services Table.
    runtime_services: *const usize,
    // TODO: A pointer to the EFI Boot Services Table. See
    // ref:efi-boot-services-table_efi_system_table.
    boot_services: *const usize,
    // TODO: The number of system configuration tables in the buffer ConfigurationTable.
    ntable_entries: usize,
    // TODO: A pointer to the system configuration tables. The number of entries in the table is
    // NumberOfTableEntries
    config_table: *const usize,
}

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    // Reset the ConsoleOut device. EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.Reset().
    reset: usize,
    // Displays the string on the device at the current cursor location.
    // EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL.OutputString() .
    output_string: fn(&Self, *const u16) -> EfiStatus,
    test_string: usize,
    query_mode: usize,
    set_mode: usize,
    set_attribute: usize,
    clear_screnn: usize,
    set_cursor_pos: usize,
    enable_cursor: usize,
    mode: usize,
}

// TODO
/*
#[repr(C)]
pub struct EfiSimpleTextInputProtocol {
    // TODO: Reset the `ConsoleIn` device. See Reset().
    reset: EfiInputReset,
    // TODO: Returnsthenextinputcharacter. SeeReadKeyStroke().
    read_key_stroke: EfiInputReadKey,
    // TODO: Event to use with EFI_BOOT_SERVICES.WaitForEvent() to wait for a key to be available.
    wait_for_key: EfiEvent,
}
*/

/// Pointer to the EFI System Table structure
static EFI_SYSTEM_TABLE: AtomicPtr<EfiSystemTable> = AtomicPtr::new(core::ptr::null_mut());


#[no_mangle]
extern "C" fn efi_main(image_hande: EfiHandle, system_table: *mut EfiSystemTable) -> EfiStatus {
    unsafe {
        /*
        let image_handle: u64;
        let system_table: u64;
        asm!(
            "mov {}, rcx",
            "mov {}, rdx",
            out(reg) image_handle,
            out(reg) system_table,
        );
        */
        let out_str = (*system_table).con_out as *const EfiSimpleTextOutputProtocol;
        let mut buffer = [0; 20];
        for (idx, utf16_chr) in "Hello World".encode_utf16().enumerate() {
            buffer[idx] = utf16_chr;
        }
        let status = ((*out_str).output_string)(out_str.as_ref().unwrap(), buffer.as_ptr());
        //let z: u64 = 0;
        //let x: u64 = (*system_table).hdr.signature as u64;
        //core::ptr::write_volatile(0x4141414141414141 as *mut u64, 0);
        panic!();
    }
    0
}

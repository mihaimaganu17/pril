// TODO: Verify checksum for the RSDP structure
// TODO: Verify ext_checksum for the RSDP structure
mod header;
mod xsdt;
mod madt;

use crate::print;
use core::mem::size_of;
pub use header::DescriptionHeader;
pub use xsdt::XSDT;
pub use madt::MADT;

/// GUID for the ACPI 2.0 vendor table, which is the RSDP structure, as reported by UEFI System
/// Table
pub const EFI_ACPI_20_TABLE_GUID: u128 = 0x81883cc7_800022bc_11d3e4f1_8868e871;

/// Root System Description Pointer Structure
#[derive(Debug)]
#[repr(C, packed)]
pub struct RSDP {
    /// The "RSD PTR" signature, that must contain a trailing blank character
    signature: [u8; 8],
    /// This is the checksum of the fields defined in the ACPI 1.0 specification. This includes only
    /// the first 20 bytes of this table, bytes 0 to 19, including the checksum field. These bytes
    /// must sum to zero
    checksum: u8,
    /// An OEM-supplied string that identifies the OEM.
    oemid: [u8; 6],
    /// The revision of this structure. Larger revision numbers are backward compatible to lower
    /// revision numbers. The ACPI version 1.0 revision number of this table is zero. The ACPI
    /// version 1.0 RSDP Structure only includes the first 20 bytes of this table, bytes 0 to 19. It
    /// does not include the Length field and beyond. The current value for this field is 2.
    revision: u8,
    /// 32 bit physical address of the RSDT.
    rsdt_addr: u32,
    /// The length of the table, in bytes, including the header, starting from offset 0. This field
    /// is used to record the size of the entire table. This field is not available in the ACPI
    /// version 1.0 RSDP Structure.
    length: u32,
    /// 64 bit physical address of the XSDT.
    xsdt_addr: u64,
    /// This is a checksum of the entire table, including both checksum fields.
    ext_checksum: u8,
    /// Reserved field
    reserved: [u8; 3],
}

/// Tries to read and return an `RSDP` structure from the `addr` pointer
pub fn read_rsdp(addr: usize) -> RSDP {
    let rsdp = unsafe {
        core::ptr::read_unaligned(addr as *const RSDP)
    };

    let signature = core::str::from_utf8(&rsdp.signature).unwrap();
    assert!("RSD PTR " == signature);

    let oemid = core::str::from_utf8(&rsdp.oemid).unwrap();

    read_acpi_table(rsdp.xsdt_addr as usize);
    read_acpi_table(rsdp.rsdt_addr as usize);

    rsdp
}

/// Reads an ACPI table
pub fn read_acpi_table(addr: usize) {
    // Read the table's header
    let header = DescriptionHeader::from_addr(addr);

    // Convert the signature into a `str` if possible
    let signature = core::str::from_utf8(&header.signature).unwrap();
    // Copy the length into a variable, because Rust cannot use it, as it was unaligned.
    let length = header.length;
    print!("Found signature: {:?} table with length: {}\n", signature, length);

    // TODO: Maybe transform this match in a list? Static list with function pointers?
    match signature {
        "XSDT" => {
            let maybe_xsdt = XSDT::from_header(addr, header);

            if let Some(xsdt) = maybe_xsdt {
                // Now that we got the XSDT, we can read the other tables, it refers to
                for table_addr in xsdt.entries.into_iter() {
                    read_acpi_table(table_addr as usize);
                }
            }
        }
        "FACP" => {
            // This is the Fixed ACPI Description Table (FADT) and it is way to fucking long to be
            // parsed at this moment. Please come back
        }
        "APIC" => {
            // This is the Multiple APIC Description Table
            let maybe_madt = MADT::from_header(addr, header);
        }
        &_ => {
            print!("Parsing for table {:?} at addr {:x?} not yet implemented!!!\n",
                signature,
                addr
            );
            todo!();
        }
    };
}


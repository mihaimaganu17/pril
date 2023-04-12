// TODO: Verify checksum for the RSDP structure
// TODO: Verify ext_checksum for the RSDP structure
use crate::print;
use core::mem::size_of;

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
    crate::print!("OEMID: {:?}\n", oemid);

    read_acpi_table(rsdp.xsdt_addr as usize);
    read_acpi_table(rsdp.rsdt_addr as usize);

    rsdp
}

/// All system description tables begin with the structure below, `DescriptionHeader`
/// The `signature` field determines the content of the system description table.
#[derive(Debug)]
#[repr(C, packed)]
pub struct DescriptionHeader {
    /// The ASCII string representation of the table identifier. Notice that if OSPM finds a
    /// signature in a table that is not listed in Table 5-29(ACPI spec, page 120),
    /// OSPM ignores the entire table (it is not loaded into ACPI namespace); OSPM ignores the
    /// table even though the values in the Length and Checksum fields are correct.
    signature: [u8; 4],
    /// The length of the table, in bytes, including the header, starting from offset 0. This field
    /// is used to record the size of the entire table.
    length: u32,
    /// The revision of the structure corresponding to the signature field for this table. Larger
    /// revision numbers are backward compatible to lower revision numbers with the same signature.
    revision: u8,
    /// The entire table, including the checksum field, must add to zero to be considered valid.
    checksum: u8,
    /// An OEM-supplied string that identifies the OEM.
    oemid: [u8; 6],
    /// An OEM-supplied string that the OEM uses to identify the particular data table. This field
    /// is particularly useful when defining a definition block to distinguish definition block
    /// functions. The OEM assigns each dissimilar table a new OEM Table ID.
    oem_table_id: [u8; 8],
    /// An OEM-supplied revision number. Larger numbers are assumed to be newer revisions.
    oem_revision: u32,
    /// Vendor ID of utility that created the table. For tables containing Definition Blocks, this
    /// is the ID for the ASL Compiler.
    creator_id: u32,
    /// Revision of utility that created the table. For tables containing Definition Blocks, this
    /// is the revision for the ASL Compiler.
    creator_revision: u32,
}

const MAX_ENTRIES: usize = 100;

/// The XSDT provides identical functionality to the RSDT but accommodates physical addresses of
/// DESCRIPTION HEADERs that are larger than 32 bits. Notice that both the XSDT and the RSDT can be
/// pointed to by the RSDP structure. An ACPI-compatible OS must use the XSDT if present.
#[repr(C)]
pub struct XSDT {
    // Header for the Table
    header: DescriptionHeader,
    // Custom structure which holds the next address in memory after the above header and a length
    // of how many entries does the XSDT contain
    entries: Entries,
}

pub struct Entries {
    addr: usize,
    length: usize,
}

/// Read the Extended System Description Table from `addr`, whith the specified length
pub fn read_xsdt(addr: usize, length: usize) -> XSDT {
    // First, read the XSDT Header
    let header = unsafe {
        core::ptr::read_unaligned(addr as *const DescriptionHeader)
    };

    let signature = core::str::from_utf8(&header.signature).unwrap();
    print!("XSDT Sig {}\n", signature);

    // Compute the number of entries followin the XSDT Table Header
    let nentries = (header.length as usize - size_of::<DescriptionHeader>()) / size_of::<u64>();

    // Since the data is not aligned, we cannot read it properly, using a
    // `core::slice::from_raw_parts` call, so we will only save the address of the list and the
    // number of entries
    XSDT {
        header,
        entries: Entries {
            addr: (addr + size_of::<DescriptionHeader>()),
            length: nentries,
        }
    }
}

// TODO: Parse all tables
pub fn read_acpi_table(addr: usize) {
    // Read the signature
    let signature = unsafe {
        core::ptr::read_unaligned(addr as *const [u8; 4])
    };

    // Read the length of the table. Some of the tables have variable lengths, so we need to read
    // this field prior to reading the table
    let length = unsafe {
        core::ptr::read_unaligned((addr + 4) as *const u32)
    };

    let signature = core::str::from_utf8(&signature).unwrap();
    print!("Found signature: {:?} table with length: {}", signature, length);

    // TODO: Maybe transform this match in a list? Static list with function pointers?
    match signature {
        "XSDT" => {
            read_xsdt(addr, length as usize),
        }
        &_ => todo!(),
    };
}


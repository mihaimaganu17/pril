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

impl DescriptionHeader {
    pub fn from_addr(addr: usize) -> Self {
        // Check if the pointer is not NULL ?

        // Read the header from the address
        let header = unsafe {
            core::ptr::read_unaligned(addr as *const DescriptionHeader)
        };

        header
    }
}

const MAX_ENTRIES: usize = 100;

/// The XSDT provides identical functionality to the RSDT but accommodates physical addresses of
/// DESCRIPTION HEADERs that are larger than 32 bits. Notice that both the XSDT and the RSDT can be
/// pointed to by the RSDP structure. An ACPI-compatible OS must use the XSDT if present.
#[repr(C, packed)]
pub struct XSDT {
    // Header for the Table
    header: DescriptionHeader,
    // Custom structure which holds the next address in memory after the above header and a length
    // of how many entries does the XSDT contain
    pub entries: Entries,
}

/// The entries that are stored in the XSDT table, specified here by their address and the legnth.
pub struct Entries {
    addr: u64,
    length: usize,
}

impl IntoIterator for Entries {
    type Item = u64;
    type IntoIter = EntriesIterator;

    fn into_iter() -> Self::IntoIter {
        EntriesIterator {
            entries: self,
            idx: 0,
        }
    }
}

/// An Iterator struct used to iterate over the records in Entries
pub struct EntriesIterator {
    entries: Entries,
    idx: usize,
}

impl Iterator for EntriesIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        // Compute the current element's address
        let elem_addr = self.entries.addr as usize + size_of::<Self::Item>() * self.idx;

        print!("Element address: {:x?}", elem_addr);

        // If there isn't enough data to read one more element, return None
        if elem_addr >= self.entries.addr as usize + self.entries.length - (size_of::<Self::Item>() + 1) {
            return None;
        }

        // Read the element
        let table_addr = unsafe { core::ptr::read_unaligned(elem_addr as *const u64) };

        // Go to the next element for the next iteration
        self.idx += 1;

        // Return the element
        Some(table_addr)
    }
}

// The signature found in the first 4 bytes from the XSDT table
const XSDT_SIGNATURE: &[u8; 4] = b"XSDT";

impl XSDT {
    /// Read the Extended System Description Table from `addr`, whith the specified length
    pub fn from_header(header: DescriptionHeader) -> Option<XSDT> {
        // Get the address of the `header` received as input
        let header_addr = header.signature.as_ptr().addr();

        // If the header's signature is no the right signature, we return `None`
        if &header.signature != XSDT_SIGNATURE {
            return None;
        }

        // Compute the number of entries followin the XSDT Table Header
        let nentries = (header.length as usize - size_of::<DescriptionHeader>()) / size_of::<u64>();

        // Since the data for the entries in the XSDT table is not aligned, we cannot read it
        // properly, using a `core::slice::from_raw_parts` call, so we will only save the address
        // of the list and the number of entries
        let xsdt = XSDT {
            header,
            entries: Entries {
                addr: (header_addr + size_of::<DescriptionHeader>()) as u64,
                length: nentries,
                idx: 0,
            }
        };

        Some(xsdt)
    }
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
            let maybe_xsdt = XSDT::from_header(header);

            if let Some(xsdt) = maybe_xsdt {
                // Now that we got the XSDT, we can read the other tables, it refers to
                for table_addr in xsdt.entries.iter() {
                    print!("Table at addr: {:x?}\n", table_addr);
                }
            }
        }
        &_ => todo!(),
    };
}


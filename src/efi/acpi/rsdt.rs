use crate::efi::acpi::DescriptionHeader;
use core::mem::size_of;

/// OSPM locates that Root System Description Table by following the pointer in the RSDP structure.
/// The RSDT, starts with the signature ‘RSDT’ followed by an array of physical pointers to other
/// system description tables that provide various information on other standards defined on the
/// current system. OSPM examines each table for a known signature. Based on the signature, OSPM
/// can then interpret the implementation-specific data within the table.
/// Platforms provide the RSDT to enable compatibility with ACPI 1.0 operating systems.
#[repr(C, packed)]
pub struct RSDT {
    // Header for the Table
    header: DescriptionHeader,
    // Custom structure which holds the next address in memory after the above header and how many
    // entries does the XSDT contain
    pub entries: Entries,
}

/// The entries that are stored in the XSDT table, specified here by their address and their
/// cardinal.
pub struct Entries {
    addr: u32,
    nentries: usize,
}

impl IntoIterator for Entries {
    type Item = u32;
    type IntoIter = EntriesIterator;

    fn into_iter(self) -> Self::IntoIter {
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
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // If there isn't enough data to read one more element, return None
        if self.idx >= self.entries.nentries {
            return None;
        }

        // Compute the current element's address
        let elem_addr = self.entries.addr as usize + size_of::<Self::Item>() * self.idx;

        // Read the element
        let table_addr = unsafe { core::ptr::read_unaligned(elem_addr as *const u32) };

        // Go to the next element for the next iteration
        self.idx += 1;

        // Return the element
        Some(table_addr)
    }
}

// The signature found in the first 4 bytes from the XSDT table
const RSDT_SIGNATURE: &[u8; 4] = b"RSDT";

impl RSDT {
    /// Read the Extended System Description Table from `addr`, whith the specified length.
    /// We also have to pass the original Physical Address from where the Header was read from,
    /// as we have to compute the start address for the entries that follow
    pub fn from_header(addr: usize, header: DescriptionHeader) -> Option<RSDT> {
        // If the header's signature is no the right signature, we return `None`
        if &header.signature != RSDT_SIGNATURE {
            return None;
        }

        // Compute the number of entries followin the XSDT Table Header
        let nentries = (header.length as usize - size_of::<DescriptionHeader>()) / size_of::<u64>();

        // Since the data for the entries in the XSDT table is not aligned, we cannot read it
        // properly, using a `core::slice::from_raw_parts` call, so we will only save the address
        // of the list and the number of entries
        let xsdt = RSDT {
            header,
            entries: Entries {
                addr: (addr + size_of::<DescriptionHeader>()) as u32,
                nentries: nentries,
            },
        };

        Some(xsdt)
    }
}

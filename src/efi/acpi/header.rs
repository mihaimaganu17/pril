/// All system description tables begin with the structure below, `DescriptionHeader`
/// The `signature` field determines the content of the system description table.
#[derive(Debug)]
#[repr(C, packed)]
pub struct DescriptionHeader {
    /// The ASCII string representation of the table identifier. Notice that if OSPM finds a
    /// signature in a table that is not listed in Table 5-29(ACPI spec, page 120),
    /// OSPM ignores the entire table (it is not loaded into ACPI namespace); OSPM ignores the
    /// table even though the values in the Length and Checksum fields are correct.
    pub signature: [u8; 4],
    /// The length of the table, in bytes, including the header, starting from offset 0. This field
    /// is used to record the size of the entire table.
    pub length: u32,
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
        let header = unsafe { core::ptr::read_unaligned(addr as *const DescriptionHeader) };

        header
    }
}

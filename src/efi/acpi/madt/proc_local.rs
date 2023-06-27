use bitflags::bitflags;
use core::fmt;
use crate::efi::acpi::madt::IntCtrlHeader;

#[repr(C, packed)]
pub(crate) struct ProcLocalApic {
    header: IntCtrlHeader,
    acpi_processor_uid: u8,
    apic_id: u8,
    flags: LocalApicFlags,
}

impl ProcLocalApic {
    /// Reads and parses a `ProcLocalApic` from a Physical Address
    pub fn from_addr(addr: usize) -> Self {
        let proc_local_apic = unsafe { core::ptr::read_unaligned(addr as *const ProcLocalApic) };

        proc_local_apic
    }
}

impl fmt::Debug for ProcLocalApic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flags = self.flags;
        f.debug_struct("ProcLocalApic")
            .field("Type", &self.header.ctrl_type)
            .field("Length", &self.header.length)
            .field("ACPI Processor Uid", &self.acpi_processor_uid)
            .field("APIC Id", &self.apic_id)
            .field("Flags", &flags)
            .finish()
    }
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    struct LocalApicFlags: u32 {
        /// If this bit is set the processor is ready for use.
        /// If this bit is clear and the Online Capable bit is set, system hardware
        /// supports enabling this processor during OS runtime.
        /// If this bit is clear and the Online Capable bit is also clear, this processor is
        /// unusable, and OSPM shall ignore the contents of the Processor Local
        /// APIC Structure.
        const ENABLED = 0b00000001;
        /// The information conveyed by this bit depends on the value of the Enabled bit.
        /// If the Enabled bit is set, this bit is reserved and must be zero.
        /// Otherwise, if this this bit is set, system hardware supports enabling this
        /// processor during OS runtime.
        const ONLINE_CAPABLE = 0b00000010;
    }
}

//! Module that parses the Local ACPI Non-Maskable Interrupt structure
use core::fmt;
use crate::efi::acpi::madt::{IntCtrlHeader, MpsIntiFlags};

/// This structure describes the Local APIC interrupt input (LINTn) that NMI is connected to for
/// each of the processors in the system where such a connection exists. This information is needed
/// by OSPM to enable the appropriate local APIC entry. Each Local APIC NMI connection requires a
/// separate Local APIC NMI structure. For example, if the platform has 4 processors with ID 0-3
/// and NMI is connected LINT1 for processor 3 and 2, two Local APIC NMI entries would be needed in
/// the MADT.
#[repr(C, packed)]
pub(crate) struct LocalApicNmi {
    header: IntCtrlHeader,
    // Value corresponding to the _UID listed in the processor’s device object, or the Processor ID
    // corresponding to the ID listed in the processor object. A value of 0xFF signifies that this
    // applies to all processors in the machine.
    acpi_proc_uid: u8,
    // MPS INTI flags.
    flags: MpsIntiFlags,
    // Local APIC Interrupt input LINTn to which NMI is connected.
    local_apic_lintn: u8,
}

impl LocalApicNmi {
    /// Reads and parses a `InOutApic` from a Physical Address
    pub fn from_addr(addr: usize) -> Self {
        let local_apic_nmi = unsafe { core::ptr::read_unaligned(addr as *const LocalApicNmi) };
        local_apic_nmi 
    }
}

impl fmt::Debug for LocalApicNmi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let flags = self.flags;

        f.debug_struct("InOutApic")
            .field("Type", &self.header.ctrl_type)
            .field("Length", &self.header.length)
            .field("ACPI Proc UID", &self.acpi_proc_uid)
            .field("Flags", &flags)
            .field("Local APIC LINT Number", &self.local_apic_lintn)
            .finish()
    }
}

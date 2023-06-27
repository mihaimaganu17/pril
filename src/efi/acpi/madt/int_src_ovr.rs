//! Module that parses the Input/Output APIC Interrupt Controller Structure
use core::fmt;
use crate::efi::acpi::madt::{IntCtrlHeader, MpsIntiFlags};

/// Describes an Interrupt Source Override structure from the MADT ACPI table.
///
/// It is assumed that the ISA interrupts will be identity-mapped into the first I/O APIC sources.
/// Most existing APIC designs, however, will contain at least one exception to this assumption.
/// The Interrupt Source Override Structure is provided in order to describe these exceptions.
///
/// For example, if your machine has the ISA Programmable Interrupt Timer (PIT) connected to ISA 
/// IRQ 0, but in APIC mode, it is connected to I/O APIC interrupt input 2, then you would need an
/// Interrupt Source Override where the source entry is ‘0’ and the Global System Interrupt is ‘2.’
#[repr(C, packed)]
pub(crate) struct IntSrcOverride {
    header: IntCtrlHeader,
    // Constant, meaning ISA
    bus: u8,
    // Bus-relative interrupt source (IRQ)
    source: u8,
    // The Global System Interrupt that this bus-relative interrupt source will
    // signal.
    global_sys_int: u32,
    // MPS INTI flags.
    flags: MpsIntiFlags,
}

impl IntSrcOverride {
    /// Reads and parses a `InOutApic` from a Physical Address
    pub fn from_addr(addr: usize) -> Self {
        let int_src_ovr = unsafe { core::ptr::read_unaligned(addr as *const IntSrcOverride) };
        int_src_ovr
    }
}

impl fmt::Debug for IntSrcOverride {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let global_sys_int = self.global_sys_int;
        let flags = self.flags;

        f.debug_struct("InOutApic")
            .field("Type", &self.header.ctrl_type)
            .field("Length", &self.header.length)
            .field("Bus", &self.bus)
            .field("Source", &self.source)
            .field("Global System Interrupt", &global_sys_int)
            .field("Flags", &flags)
            .finish()
    }
}

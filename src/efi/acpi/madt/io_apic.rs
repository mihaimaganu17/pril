//! Module that parses the Input/Output APIC Interrupt Controller Structure
use core::fmt;
use crate::efi::acpi::madt::IntCtrlHeader;

/// In an APIC implementation, there are one or more I/O APICs. Each I/O APIC has a series of
/// interrupt inputs, referred to as INTIn, where the value of n is from 0 to the number of the
/// last interrupt input on the I/O APIC. The I/O APIC structure declares which global system
/// interrupts are uniquely associated with the I/O APIC interrupt inputs. There is one I/O APIC
/// structure for each I/O APIC in the system
#[repr(C, packed)]
pub(crate) struct InOutApic {
    header: IntCtrlHeader,
    // I/O APIC Id
    io_apic_id: u8,
    // Reserved, must be 0
    reserved: u8,
    // The 32-bit physical address to access this I/O APIC. Each I/O APIC resides
    // at a unique address.
    io_apic_addr: u32,
    // The global system interrupt number where this I/O APIC’s interrupt
    // inputs start. The number of interrupt inputs is determined by the I/O
    // APIC’s Max Redir Entry register.
    global_system_int_base: u32,
}

impl InOutApic {
    /// Reads and parses a `InOutApic` from a Physical Address
    pub fn from_addr(addr: usize) -> Self {
        let io_apic = unsafe { core::ptr::read_unaligned(addr as *const InOutApic) };
        io_apic
    }
}

impl fmt::Debug for InOutApic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let io_apic_addr = self.io_apic_addr;
        let global_system_int_base = self.global_system_int_base;

        f.debug_struct("InOutApic")
            .field("Type", &self.header.ctrl_type)
            .field("Length", &self.header.length)
            .field("I/O APIC Id", &self.io_apic_id)
            .field("I/O APIC Address", &io_apic_addr)
            .field("Global System Interrupt Base", &global_system_int_base)
            .finish()
    }
}

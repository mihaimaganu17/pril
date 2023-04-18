use core::mem::size_of;
use crate::print;
use crate::efi::acpi::DescriptionHeader;

const MAX_INT_CTRLS: usize = 200;

/// Multiple APIC Description Table.
/// This is the ACPI way to describe all interrupts from the entire system in an uniform interrupt
/// model implementation. The supported interrupt models include:
/// - The PC-AT-compatible dual 8259 Interrupt controller
/// - Intel Advanced Programmable Interrupt Controller (APIC)
/// - The Streamlined version of the controller above (SAPIC)
/// - The Generic Interrupt Controller(GIC) for ARM systems.
/// If a platform supports multiple models, an OS will install support for only one of the models;
/// it will not mix models.
/// ACPI represents all interrupts as "flat" values known as global system interrupts.
/// Therefore to support APICs, SAPICs or GICs on an ACPI-enabled system, each used interrupt
/// input must be mapped to the global system interrupt value used by ACPI.
/// All addresses in the MADT are processor-relative physical addresses.
#[repr(C, packed)]
#[derive(Copy)]
pub struct MADT {
    header: DescriptionHeader,
    // Local Interrupt Controller Address
    // The 32-bit physical address at which each processor can access its local interrupt
    // controller.
    lic_addr: u32,
    // Multiple APIC flags.
    flags: u32,
    // A list of interrupt controller structures that declare the interrupt features of the
    // machine. The first byte of each structure decalres the type of that structure and the second
    // byte declared the length of that structure.
    int_ctrls: [Option<IntCtrl>; MAX_INT_CTRLS],
}

impl MADT {
    pub fn from_header(addr: usize, header: DescriptionHeader) -> Option<Self> {
        // Compute the address after the header of the MADT
        let after_header_addr = addr + size_of::<DescriptionHeader>();

        // Read the Local Interrupt controller and the flags
        let (lic_addr, flags): (u32, u32) = unsafe {
            (
                core::ptr::read_unaligned(after_header_addr as *const u32),
                core::ptr::read_unaligned((after_header_addr + size_of::<u32>()) as *const u32),
            )
        };

        // Get the length of the entire structure
        let madt_end_addr = addr + header.length as usize;

        // Compute the address for the next Interrupt Controller
        let mut next_int_ctrl_addr = after_header_addr + 2 * size_of::<u32>();

        // As long as we still have data to read, we read it
        while next_int_ctrl_addr < madt_end_addr {
            // Read the Metadata for the controller
            let int_ctrl_meta = unsafe {
                core::ptr::read_unaligned(next_int_ctrl_addr as *const IntCtrlHeader)
            };

            let int_ctrl = IntCtrl::from_type(next_int_ctrl_addr, int_ctrl_meta.ctrl_type);

            // Print it for debugging
            print!("Int ctrl: {} with length {}\n", int_ctrl_meta.ctrl_type, int_ctrl_meta.length);

            // Update the address to read the next Interrupt Controller
            next_int_ctrl_addr += int_ctrl_meta.length as usize;
        }

        let madt = MADT {
            header,
            lic_addr,
            flags,
            int_ctrls: [None; MAX_INT_CTRLS],
        };

        Some(madt)
    }
}

/// Represents common fields for any Interrupt controller that we will refer to as a header.
#[repr(C, packed)]
struct IntCtrlHeader{
    // Type of the structures
    ctrl_type: u8,
    // The length of the structure
    length: u8,
}

/// Represents supported interrupt controller types
pub enum IntCtrl {
    ProcLocalApic,
    InOutApic,
    IntSrcOverride,
    NmiSrc,
    LocalApicNmi,
    Unknown(u8),
}

mod IntCtrlType {
    pub const PROCESSOR_LOCAL_APIC: u8 = 0;
    pub const IN_OUT_APIC: u8 = 1;
    pub const INTERRUPT_SOURCE_OVERRIDE: u8 = 2;
    pub const NON_MASKABLE_INTERRUPT: u8 = 3;
    pub const LOCAL_APIC_NMI: u8 = 4;
}

#[repr(C, packed)]
struct ProcLocaApic {
    header: IntCtrlHeader,
    acpi_processor_uid: u8,
    apic_id: u8,
    flags: u32,
}

impl IntCtrl {
    pub fn from_type(addr: usize, ctrl_type: u8) -> Self {
        match ctrl_type {
            IntCtrlType::PROCESSOR_LOCAL_APIC => {
            }
        }
    }
}

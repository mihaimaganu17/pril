//! Module that holds the Interrupt Controller parsing logic and structures
use crate::efi::acpi::madt::{
    proc_local::ProcLocalApic,
    io_apic::InOutApic,
    int_src_ovr::IntSrcOverride,
    local_apic_nmi::LocalApicNmi,
};
use bitflags::bitflags;

/// Represents common fields for any Interrupt controller that we will refer to as a header.
#[repr(C, packed)]
pub(crate) struct IntCtrlHeader {
    // Type of the structures
    pub ctrl_type: u8,
    // The length of the structure
    pub length: u8,
}

/// Represents supported interrupt controller types
#[derive(Debug)]
pub(crate) enum IntCtrl {
    ProcLocalApic(ProcLocalApic),
    InOutApic(InOutApic),
    IntSrcOverride(IntSrcOverride),
    NmiSrc,
    LocalApicNmi(LocalApicNmi),
    Unknown(u8),
}

impl IntCtrl {
    pub fn from_type(addr: usize, ctrl_type: u8) -> Self {
        match ctrl_type {
            int_ctrl_type::PROCESSOR_LOCAL_APIC => {
                Self::ProcLocalApic(ProcLocalApic::from_addr(addr))
            }
            int_ctrl_type::IN_OUT_APIC => {
                Self::InOutApic(InOutApic::from_addr(addr))
            }
            int_ctrl_type::INTERRUPT_SOURCE_OVERRIDE => {
                Self::IntSrcOverride(IntSrcOverride::from_addr(addr))
            }
            int_ctrl_type::NON_MASKABLE_INTERRUPT => {
                Self::NmiSrc
            }
            int_ctrl_type::LOCAL_APIC_NMI => {
                Self::LocalApicNmi(LocalApicNmi::from_addr(addr))
            }
            _ => Self::Unknown(ctrl_type),
        }
    }
}

mod int_ctrl_type {
    pub const PROCESSOR_LOCAL_APIC: u8 = 0;
    pub const IN_OUT_APIC: u8 = 1;
    pub const INTERRUPT_SOURCE_OVERRIDE: u8 = 2;
    pub const NON_MASKABLE_INTERRUPT: u8 = 3;
    pub const LOCAL_APIC_NMI: u8 = 4;
}

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct MpsIntiFlags: u16 {
        // Polarity of the APIC I/O input signals
        const ACTIVE_HIGH = 0b00000001;
        const POLARITY_RESERVED = 0b00000010;
        const ACTIVE_LOW = 0b00000011;

        // Trigger mode of the APIC I/O Input signals
        const EDGE_TRIGGERED = 0b00000100;
        const TRIGGER_RESERVED = 0b00001000;
        const LEVEL_TRIGGERED = Self::EDGE_TRIGGERED.bits() | Self::TRIGGER_RESERVED.bits();
    }
}

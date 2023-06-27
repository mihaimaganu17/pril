use core::arch::asm;
use bitflags::bitflags;

pub unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, {value}",
        value = in(reg_byte) value,
        in("dx") port,
    );
}

pub unsafe fn inb(port: u16) -> u8 {
    let value;
    asm!(
        "in {value}, dx",
        value = out(reg_byte) value,
        in("dx") port,
    );
    value
}

/// Typically, Control Registers are the same size as the underlying mode that they run on,
/// (either 32-bits or 64-bits). Since we do not care about 32-bits right now, we will take the
/// full value.
pub unsafe fn cr0() -> CR0 {
    let value: u64;
    asm!(
        "mov {value}, cr0",
        value = out(reg) value,
    );
    CR0::from_bits(value).unwrap()
}

/// Function that read a Intel's model-specific register specified in `ecx`. Currently there is no
/// guarantee about calling this function and receiving a valid result, so you should check that
/// you are on a proper platform and in a proper CPU mode yourself. Since we are only targeting
/// IA32e, we will return a u64.
pub unsafe fn rdmsr(ecx: u32) -> u64 {
    let eax: u32;
    let edx: u32;
    asm!(
        "rdmsr",
        in("ecx") ecx,
        out("eax") eax,
        out("edx") edx,
    );
    // The RDMSR instruction reads the MSR value into EDX:EAX, where EDX represents the high 32
    // bits and EAX the low 32 bits
    let value: u64 = (edx as u64) << 32 | (eax as u64);
    value
}

/// Module that contains constants representing register addresses for Model Specific Registers.
pub mod msr_reg_addr {
    // Extended feature Enables
    pub const IA32_EFER: u32 = 0xC000_0080;
}

bitflags! {
    /// Structure represents all the informational bits from the CR0 register. If a bit is not
    /// explicitly stated here, it means it is reserved by the processor.
    #[derive(Debug)]
    pub struct CR0: u64 {
        // Protected Mode Enable
        const PE = 0x0000_0000_0000_0001;
        // Monitor Co-Processor
        const MP = 0x0000_0000_0000_0002;
        // Emulation
        const EM = 0x0000_0000_0000_0004;
        // Task Switched 
        const TS = 0x0000_0000_0000_0008;
        // Extension Type 
        const ET = 0x0000_0000_0000_0010;
        // Numeric Error
        const NE = 0x0000_0000_0000_0020;
        // Write Protect
        const WP = 0x0000_0000_0001_0000;
        // Alignment Mask
        const AM = 0x0000_0000_0004_0000;
        // Not-Write Through
        const NW = 0x0000_0000_2000_0000;
        // Cache Disable
        const CD = 0x0000_0000_4000_0000;
        // Paging
        const PG = 0x0000_0000_8000_0000;
    }
}

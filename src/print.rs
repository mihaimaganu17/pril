//! Module that holds the print macro
use core::arch::asm;
use core::sync::atomic::{AtomicPtr, Ordering};

// Dummy writer we can implement `Write` trait on, so that we can support formatted strings
pub struct ConsoleOutWriter;

impl core::fmt::Write for ConsoleOutWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Calls our own UEFI implementation for outputting a string to the console
        crate::efi::uefi_print(s);
        Ok(())
    }
}

/// Know address for the COM1 serial port(should be taken from the BDA)
pub const PORT_COM1: u16 = 0x3f8;

/// Static pointer that hold the COM port pointer we want to write to
pub static SERIAL_PORT: AtomicPtr<u16> = AtomicPtr::new(core::ptr::null_mut());

pub struct SerialWriter;

impl core::fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.as_bytes() {
            crate::print::SerialWriter::transmit(*byte);
        }
        Ok(())
    }
}

impl SerialWriter {
    pub fn init_serial(port: &mut u16) {
        SERIAL_PORT
            .compare_exchange(
                core::ptr::null_mut(),
                port,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .unwrap();

        let port = SERIAL_PORT.load(Ordering::SeqCst);

        if port.is_null() {
            return;
        }

        unsafe {
            outb(*port + 1, 0x00); // Disable all interrupts
            outb(*port + 3, 0x80); // Enable DLAB (set baud rate divisor)
            outb(*port + 0, 0x01); // Set divisor to 1 (lo byte) 115200 baud
            outb(*port + 1, 0x00); //                  (hi byte)
            outb(*port + 3, 0x03); // 8 bits, no parity, one stop bit
            outb(*port + 2, 0xC7); // Enable FIFO, clear them, with 14-byte threshold
            outb(*port + 4, 0x0B); // IRQs enabled, RTS/DSR set
            outb(*port + 4, 0x1E); // Set in loopback mode, test the serial chip
            outb(*port + 0, 0xAE); // Test serial chip
                                   // (send byte 0xAE and check if serial returns same byte)
        };

        // Check if serial is faulty (i.e: not same byte as sent)
        if (unsafe { inb(*port + 0) } != 0xAE) {
            // We just keep this print here for debug purposes
            crate::print!("Faulty");
            return;
        }

        // If serial is not faulty set it in normal operation mode
        // (not-loopback with IRQs enabled and OUT#1 and OUT#2 bits enabled)
        unsafe {
            outb(*port + 4, 0x0F);
        }
    }

    pub fn transmit(byte: u8) {
        let port = SERIAL_PORT.load(Ordering::SeqCst);

        if port.is_null() {
            return;
        }

        unsafe {
            while (inb(*port + 5) & 0x20) == 0 {}
            outb(*port, byte);
        }
    }
}

unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, {value}",
        value = in(reg_byte) value,
        in("dx") port,
    );
}

unsafe fn inb(port: u16) -> u8 {
    let value;
    asm!(
        "in {value}, dx",
        value = out(reg_byte) value,
        in("dx") port,
    );
    value
}

#[macro_export]
macro_rules! print {
    ( $($arg:tt)* ) => {
        // Get the COM port pointer
        let port = $crate::print::SERIAL_PORT.load(core::sync::atomic::Ordering::SeqCst);

        // If the port is null, initialize the port
        if port.is_null() {
            let mut port_com1 = $crate::print::PORT_COM1;
            $crate::print::SerialWriter::init_serial(&mut port_com1);
        }
        let mut serial_writer = $crate::print::SerialWriter;
        core::fmt::write(&mut serial_writer, core::format_args!($($arg)*)).unwrap();
    }
}

#[macro_export]
macro_rules! print_uefi {
    ( $($arg:tt)* ) => {
        let mut console_writer = $crate::print::ConsoleOutWriter;
        core::fmt::write(&mut console_writer, core::format_args!($($arg)*)).unwrap();
    }
}

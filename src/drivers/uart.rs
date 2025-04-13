use embedded_hal::serial::{Write, Read};
use nb::block;
use cortex_m::asm;

pub struct Uart {
    // UART registers will be added here
    // For now, we'll use a placeholder
    _registers: u32,
}

impl Uart {
    pub fn new() -> Self {
        // Initialize UART hardware
        // This is a placeholder - actual implementation will depend on your MCU
        Self {
            _registers: 0,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        // Placeholder for actual UART write
        // In real implementation, this would write to UART registers
        asm::delay(1000); // Simulate some delay
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
    }
}

impl Write<u8> for Uart {
    type Error = ();

    fn write(&mut self, byte: u8) -> Result<(), Self::Error> {
        self.write_byte(byte);
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        // Wait for transmission to complete
        Ok(())
    }
} 

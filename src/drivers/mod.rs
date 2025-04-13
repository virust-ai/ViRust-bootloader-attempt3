pub mod uart;
pub mod can;
pub mod flash;

pub use uart::Uart;
pub use can::CanDriver;
pub use flash::FlashDriver;

pub struct CanDriver {
    // CAN controller registers and state
}

impl CanDriver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn send_message(&mut self, msg: &[u8]) -> Result<(), DriverError> {
        // Implement CAN message sending
        Ok(())
    }

    pub fn receive_message(&mut self) -> Result<Vec<u8>, DriverError> {
        // Implement CAN message receiving
        Ok(Vec::new())
    }
}

pub struct FlashDriver {
    // Flash controller registers and state
}

impl FlashDriver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn write_page(&mut self, address: u32, data: &[u8]) -> Result<(), DriverError> {
        // Implement flash page writing
        Ok(())
    }

    pub fn read_page(&self, address: u32) -> Result<Vec<u8>, DriverError> {
        // Implement flash page reading
        Ok(Vec::new())
    }

    pub fn erase_page(&mut self, address: u32) -> Result<(), DriverError> {
        // Implement flash page erasing
        Ok(())
    }
} 

#![no_std]
#![no_main]

mod startup;
mod bootloader;
mod security;
mod protocol;
mod drivers;

use cortex_m_rt::entry;
use panic_halt as _;
use drivers::Uart;

#[entry]
fn main() -> ! {
    // Initialize UART
    let mut uart = Uart::new();
    
    // Print boot message
    uart.write_str("Bootloader starting...\r\n");
    
    // The actual bootloader logic will be in the bootloader module
    bootloader::run()
} 

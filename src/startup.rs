#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn bootloader_entry() -> ! {
    // Initialize critical hardware
    let mut bootloader = bootloader::Bootloader::new();
    
    // Perform secure boot checks
    if bootloader.verify_secure_boot().is_err() {
        // Handle secure boot failure
        panic!("Secure boot failed");
    }
    
    // Check if we need to update firmware
    if bootloader.check_update_needed() {
        bootloader.enter_update_mode();
    } else {
        // Jump to application
        bootloader.jump_to_application();
    }
    
    loop {
        // Should never reach here
        cortex_m::asm::bkpt();
    }
} 

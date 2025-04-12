#![no_std]
#![no_main]

mod startup;
mod bootloader;
mod security;
mod protocol;
mod drivers;

use cortex_m_rt::entry;
use panic_halt as _;

#[entry]
fn main() -> ! {
    // The actual bootloader logic will be in the bootloader module
    bootloader::run()
} 

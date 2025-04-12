/* Linker script for bootloader */
MEMORY
{
  /* Bootloader section */
  BOOTLOADER (rx) : ORIGIN = 0x00000000, LENGTH = 32K
  
  /* Application sections */
  APP_FLASH (rx) : ORIGIN = 0x00008000, LENGTH = 224K
  APP_RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 64K
  
  /* DFU section for firmware updates */
  DFU_FLASH (rx) : ORIGIN = 0x00010000, LENGTH = 224K
  
  /* Bootloader state section */
  BOOT_STATE (r) : ORIGIN = 0x00007000, LENGTH = 4K
} 

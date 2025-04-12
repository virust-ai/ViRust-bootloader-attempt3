# Rust Bootloader

A secure bootloader implementation in Rust with support for firmware updates via XCP/UDS over CAN.

## Features

- Secure boot with cryptographic verification
- Firmware updates via XCP protocol
- UDS diagnostic support
- Power-fail safe updates
- Anti-rollback protection

## Project Structure

```
rust-bootloader/
├── .cargo/              # Cargo configuration
├── src/                 # Source code
│   ├── bootloader/     # Bootloader core
│   ├── security/       # Security features
│   ├── protocol/       # XCP/UDS protocols
│   └── drivers/        # Hardware drivers
├── memory.x            # Linker script
└── build.rs           # Build script
```

## Building

1. Install required tools:
```bash
rustup target add thumbv7em-none-eabihf
```

2. Build the project:
```bash
cargo build --target thumbv7em-none-eabihf
```

## Memory Layout

- Bootloader: 32KB at 0x00000000
- Application: 224KB at 0x00008000
- DFU: 224KB at 0x00010000
- Boot State: 4KB at 0x00007000

## License

This project is licensed under the MIT License - see the LICENSE file for details. 

# Writing a HAL from scratch

* SVD Files
    * We "fixed" some bugs
    * (probably need to check this for correctness later)
* `svd2rust`
    * Generated code
    * Used `form` to split up the code
    * Used `cargo fmt` and `rustfmt` to prettify the code
* PAC - Peripheral Access Crate
    * We had to add some dependencies
    * Looked to other crates for inspiration
    * Found our crate was for a `thumbv7m-none-eabi` target
        * Cortex-M0/M0+: thumbv6m-none-eabi
        * Cortex-M3: thumbv7m-none-eabi
        * Cortex-M3 w/ HF: thumbv7m-none-eabihf
        * Cortex-M4/M7: thumbv7em-none-eabi
        * Cortex-M4F/M7F: thumbv7em-none-eabihf

## Fork in the road

### Long term goal: HAL crate

You use a PAC to hand-write a HAL

### Today's goal

Just an application that uses the PAC directly

Next steps:

* Set up an application that uses the PAC
* Pull up the datasheet
* Get a basic main running
* Get a debugger attached, flash code

## Set up an Application

* Make a new project with `cargo new --bin $name`
* Create a `memory.x`
    * Go find "Memory Map" in the datasheet
    * Make sure you have `FLASH` and `RAM` locations and amounts correct
    * Used for the linker script
* Create a `.cargo/config`
    * Setup automation
    * Set default target
    * Tell cargo where to look for the linker script
* Create a `debug.gdb`
* Add a panic handler crate
* Edit `src/main.rs`
    * Don't forget to include your pac

GET THE DEBUGGER RUNNING

## Using the PAC interfaces

Useful for reading, writing, and modifying registers

VOCAB FOR PACs

* READ: Means **read** from the register
    * Interact with the register ONCE
* MODIFY: Means **read**, make **changes**, then **write**
    * Interact with the register TWICE
* WRITE: Means **take the default value**, make **changes**, then **write**
    * Interact with the register ONCE

Important Notes:

* Not all registers have all interfaces
* Always start with the `Peripherals` struct from your PAC


# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

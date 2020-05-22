#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use cc13x0_pac as _;

#[entry]
fn main() -> ! {
    loop {
        continue;
    }
}

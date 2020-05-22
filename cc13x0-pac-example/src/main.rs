#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m_rt::entry;
use cc13x0_pac::Peripherals;

#[entry]
fn main() -> ! {

    let peripherals = Peripherals::take().unwrap();

    // THIS IS OUR register group, `FCFG1` from the datasheet
    let fcfg = peripherals.FCFG1;

    let mac0: u32 = fcfg
        // This is an individual register, `mac_15_4_0`.
        // We want to read from it.
        .mac_15_4_0.read()

        // This is getting the subfield, which is part
        // of the full 32 bit register. Get the contents
        // of this register as "bits" (or a u32)
        .addr_0_31().bits();

    let mac1: u32 = fcfg
        .mac_15_4_1.read()
        .addr_32_63().bits();

    let aon = Rtc::new(peripherals.AON_RTC);

    // Lets enable the clock

    aon.enable();

    loop {

        if aon.now_seconds() >= 3 {
            let x = 0;

            aon.reset();
        }
    }

    // Terminal loop
    loop {
        continue;
    }
}

struct Rtc {
    aon: cc13x0_pac::AON_RTC
}

impl Rtc {
    fn new(aon: cc13x0_pac::AON_RTC) -> Self {
        Self { aon }
    }

    fn enable(&self) {
        self.aon.ctl.write(|w| {
            w.en().set_bit()
        });
    }

    fn reset(&self) {
        // Reset our counter and wait
        self.aon.ctl.modify(|_r, w| {
            w.reset().set_bit()
        });

        while self.aon.ctl.read().reset().bit_is_set() && self.aon.sec.read().bits() != 0 {
            continue;
        }
    }

    fn now_seconds(&self) -> u32 {
        self.aon.sec.read().bits()
    }
}


#![no_std]
#![no_main]

use panic_halt as _;

// mod interrupts;

use cortex_m_rt::entry;
use stm32l0::stm32l0x3::Peripherals;

#[entry]
fn main() -> ! {
    let mut ph = Peripherals::take().unwrap();

    ph.GPIOA.odr.write(|port| port.bits(5));

    loop {
        
    }
}

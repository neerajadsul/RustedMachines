#![no_std]
#![no_main]

use core::arch::asm;

use panic_halt as _;

// mod interrupts;

use cortex_m_rt::entry;
use stm32l0::stm32l0x3::Peripherals;

#[entry]
unsafe fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;

    gpioa.odr.write(|port| port.bits(5));

    loop {
        for _ in 0..u32::pow(2, 16){
            asm!("nop");
        }
        
    }
}

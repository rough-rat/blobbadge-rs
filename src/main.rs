
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
// #![feature(generic_const_exprs)]

use embassy_time::Timer;
use embassy_executor::Spawner;

use py32_hal::{gpio::Pin, Peripheral};

use defmt::*;
use {defmt_rtt as _, panic_halt as _};
// use cortex_m_rt::entry;

use py32_hal::gpio::Flex;

use py32_hal::gpio::AnyPin;

// mod charlie;
mod utils;
mod bat;


async fn charlie_simple<const S: usize>(pin_list: [AnyPin; S]) {
    unsafe{
        let mut gpio_list = {
            let mut list: [Flex<'_>; S] = core::mem::MaybeUninit::uninit().assume_init();
            for i in 0..S {
                list[i] = Flex::new(pin_list[i].clone_unchecked());
            }
            list
        };

        loop {
            for p in gpio_list.iter_mut() {
                utils::set_random(p);
                Timer::after_millis((0 + utils::bootleg_random() % 30).into()).await; 
            }
            // Timer::after_millis(500).await;  
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");
    let p = py32_hal::init(Default::default());

    _spawner.spawn(bat::run_bat_monitor(p.ADC)).unwrap();

    let led_pins = [
        p.PA0.degrade(),
        p.PA1.degrade(),
        p.PA2.degrade(),
        p.PA3.degrade(),
        p.PA4.degrade(),
        p.PA5.degrade(),
        p.PA6.degrade(),
    ];
    
    charlie_simple(led_pins).await;
    let mut cnt:u32 = 0;

    loop{
        info!("tick {}", cnt);
        cnt += 1;
        Timer::after_millis(5000).await;
    }
    // bat::run_adc(&mut p);

}


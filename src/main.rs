
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(generic_const_exprs)]

use embassy_time::Timer;
use embassy_executor::Spawner;

use py32_hal::{gpio::Pin, Peripheral};

use defmt::*;
use {defmt_rtt as _, panic_halt as _};
// use cortex_m_rt::entry;


mod charlie;
mod utils;
mod bat;



#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");
    let p = py32_hal::init(Default::default());

    _spawner.spawn(bat::run_bat_monitor(p.ADC)).unwrap();

    let red_pins = [
        p.PA0.degrade(),
        p.PA1.degrade(),
        p.PA2.degrade(),
        p.PA3.degrade(),
        p.PA4.degrade(),
        p.PA5.degrade(),
        p.PA6.degrade(),
    ];

    let white_pins = [
        p.PA7.degrade(),
        p.PA8.degrade(),
        p.PA12.degrade(),
        p.PA13.degrade(),
    ];
    
    // charlie::charlie_simple_loop(led_pins).await;

    let mut cr = charlie::Charlie::new(red_pins);
    // let mut cw = charlie::Charlie::new(white_pins);
    
    let mut cnt:u32 = 0;
    // cr.buf[0] = 0x01;
    cr.buf[33] = 0x01;
    loop{

        // cr.draw();
        // cw.draw_random();
        cr.draw().await;

        cnt += 1;

        // cr.buf[(cnt % 32)as usize] = 0x01;
        if cnt % 1000 == 0 {
            info!("tick {}", cnt);
        }

        
    }
    // bat::run_adc(&mut p);

}


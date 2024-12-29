
#![no_std]
#![no_main]
#![feature(impl_trait_in_assoc_type)]
#![feature(generic_const_exprs)]

use cortex_m::peripheral::SCB;
use cortex_m_rt::{exception, ExceptionFrame};
use embassy_time::{Ticker, Duration, Instant};
use embassy_executor::Spawner;

use py32_hal::gpio::Pin;

use py32_hal::rcc::{Pll, PllSource, Sysclk};
use py32_hal::time::Hertz;
use defmt::*;
use utils::{bootleg_random, bootleg_random_u8};
use {defmt_rtt as _, panic_halt as _};

mod charlie;
mod utils;
mod bat;

const ANIM_NUM: u8 = 1;
const REDRAW_CNT :u16 = 10;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Hello World!");

    let mut cfg: py32_hal::Config = Default::default();
    cfg.rcc.hsi = Some(Hertz::mhz(24));
    cfg.rcc.pll = Some(Pll {
        src: PllSource::HSI,
    });
    cfg.rcc.sys = Sysclk::PLL;
    let p = py32_hal::init(cfg);

    // _spawner.spawn(bat::run_bat_monitor(p.ADC)).unwrap();

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
    
    let mut cr = charlie::Charlie::new(red_pins);
    // #[cfg(allow_debug_probe)] //todo
    let mut cw = charlie::Charlie::new(white_pins);

    let mut cnt:u16 = 0;
    let mut ticker = Ticker::every(Duration::from_millis(20));

    loop{
        let time_start: Instant = Instant::now();
        
        match ANIM_NUM {
            0 => {
                cr.effect_ember();
                cw.effect_ember();
            }
            _ => {
                cr.set_by_offs(
                    cnt, 
                    (cr.get_by_offs(cnt) + 32) % 128
                );
                cw.set_by_offs(
                    cnt, 
                    (cw.get_by_offs(cnt) + 32) % 128
                );
            }
        }

        for _ in 0..REDRAW_CNT {
            cr.draw().await;
            cw.draw().await;
        }

        cnt += 1;
        if cnt % 10 == 0 {
            let duration: Duration = Instant::now() - time_start;
            info!("tick {}, render time {}us", cnt, duration.as_micros());
        }
        ticker.next().await;        
    }
}

//conserve flash space with the non-handler
#[exception]
unsafe fn HardFault(_frame: &ExceptionFrame) -> ! {
    SCB::sys_reset()
}


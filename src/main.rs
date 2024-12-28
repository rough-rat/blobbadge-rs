
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


fn mod_clip(in_data: u8, modulus: u8) -> u8 {
    let t: i32 = in_data as i32
         + (bootleg_random_u8() % 3) as i32 - 1;
    if t > modulus as i32 {
        return modulus -1;
    };
    if t < 0  {
        return 0;
    }
    return t as u8
    // }
}

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
    // #[cfg(allow_debug_probe)] //todo
    let mut cw = charlie::Charlie::new(white_pins);

    let mut cnt:u16 = 0;
    let mut ticker = Ticker::every(Duration::from_millis(20));

    loop{
        let time_start: Instant = Instant::now();
        // cr.draw();
        // cw.draw_random();
        cr.draw().await;
        cr.draw().await;
        // cr.draw().await;
        // cr.draw().await;
        cr.draw().await;
        cw.draw().await;

        cnt += 1;
        
        cr.set_by_offs(
            usize::from(cnt) % cr.buf_size(),
            (cr.buf[usize::from(cnt) % cr.buf_size()] + bootleg_random_u8()%16 - 8) % 3
        );

        // cw.set_by_offs(
        //     usize::from(cnt) % cr.buf_size(),
        //     (cr.buf[usize::from(cnt) % cr.buf_size()] + bootleg_random_u8()%16 - 8) % 3
        // );


        // for i in 0..cr.buf_size() {
        //     cr.set_by_offs(
        //         i,
        //         (cr.buf[i] + bootleg_random_u8()%3 - 1) % 128
        //     );
        // }

        // for i in 0..cr.buf_size() {
        //     cr.set_by_offs(
        //         i,
        //         mod_clip(cr.buf[i],2)
        //     );
        // }
        // cr.set_by_offs(
        //     usize::from(cnt) % cr.buf_size(),
        //     (cr.buf[usize::from(cnt) % cr.buf_size()] + bootleg_random_u8()%16 - 8) % 128
        // );


        // cw.set_by_offs(
        //     usize::from(cnt) % cw.buf_size(),
        //     (cr.buf[usize::from(cnt) % cw.buf_size()] + 32) % 128
        // );

        // (cr.buf[usize::from(cnt) % cr.buf_size()] + 1)%2_u8.pow(charlie::BIT_DEPTH.into())

        // if cnt % 32 == 0 {
            // cr.buf[((cnt) % 42)as usize] = !cr.buf[((cnt) % 42)as usize];
            // cw.buf[((cnt) % 12)as usize] = !cw.buf[((cnt) % 12)as usize];
        // }
        
        if cnt % 10 == 0 {
            let duration: Duration = Instant::now() - time_start;
            info!("tick {}, render time {}us", cnt, duration.as_micros());
        }
        ticker.next().await;        
    }
    // bat::run_adc(&mut p);

}

#[exception]
unsafe fn HardFault(_frame: &ExceptionFrame) -> ! {
    SCB::sys_reset() // <- you could do something other than reset
}


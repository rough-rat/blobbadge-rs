
use py32_hal::{gpio::{Flex, AnyPin}, Peripheral};
use embassy_time::{Ticker, Duration, Instant};
use defmt::*;

use embassy_time::Timer;

use crate::utils;

// awful i know
const STATE_LOW: u8 = 0;
const STATE_HIGH: u8 = 1;
const STATE_TRI: u8 = 2;

pub const BIT_DEPTH: u8 = 5;
pub const TARGET_FPS: u32 = 60;

//the delay base is calculated for a single row on of single charlieplex object
const DELAY_DERATING: u32 = 5;  

// frame_time = 1/fps
// base = frame_time / 2^BIT_DEPTH
// 1e6 / 60*256*16 = 4
const DELAY_BASE_US: u32 = 
    1_000_000 / (DELAY_DERATING * TARGET_FPS * 3_u32.pow(BIT_DEPTH as u32)); 
// const DELAY_BASE: u32 = 100;


#[allow(dead_code)]
pub fn set_random(pin: &mut Flex<'_>) {
    match utils::bootleg_random() % 3 {
        0 => {
            pin.set_as_output(py32_hal::gpio::Speed::Low);
            pin.set_low();
        }
        1 => {
            pin.set_as_output(py32_hal::gpio::Speed::Low);
            pin.set_high();
        }
        _ => pin.set_as_input(py32_hal::gpio::Pull::None)
    }
}


pub struct Charlie<'a, const PIN_COUNT: usize> 
    where [(); PIN_COUNT * (PIN_COUNT-1)]:
{
    pub pin_list: [Flex<'a>; PIN_COUNT],
    pub pin_state: [u8; PIN_COUNT],
    pub buf: [u8; PIN_COUNT * (PIN_COUNT-1)]
}


impl <'a, const PIN_COUNT: usize> Charlie<'a, {PIN_COUNT} >
    where [(); PIN_COUNT * (PIN_COUNT-1)]:
{
    pub fn new(pin_list: [AnyPin; PIN_COUNT])
        -> Charlie<'a, PIN_COUNT>
    {
        unsafe{
            let gpio_list = {
                let mut list: [Flex<'_>; PIN_COUNT] = 
                    core::mem::MaybeUninit::uninit().assume_init();

                for i in 0..PIN_COUNT {
                    list[i] = Flex::new(pin_list[i].clone_unchecked());
                }
                list
            };

            let c = Charlie {
                pin_list: gpio_list,
                pin_state: [0; PIN_COUNT],
                buf: [0; PIN_COUNT * (PIN_COUNT-1)]
            };

            return c
        }
    }

    pub fn pin_count(&self) -> usize {
        PIN_COUNT
    }

    pub fn buf_size(&self) -> usize {
        PIN_COUNT * (PIN_COUNT-1)
    }

    pub fn set_by_offs(&mut self, offs: usize, val: u8) {
        // assert!(offs < self.buf_size());
        self.buf[offs] = val;
    }

    // fn set(&mut self, row: u8, col: u8, val: u8) {
    //     self.buf[row as usize * (PIN_COUNT -1) + col as usize] = val;
    // }

    pub fn get(&mut self, row: u8, col: usize) -> &mut u8 {
        // self.buf[row as usize * (PIN_COUNT -1) + col as usize]
        &mut self.buf[row as usize * (PIN_COUNT -1) + col as usize]
    }

    // https://embassy.dev/book/#_executor
    async fn delay (&self, time_us: u32) {
        // if time_ns < 1000 {
        //     //100cycles @ 24MHz = 4us 
        //     cortex_m::asm::delay(time_ns);
        // } else {
            Timer::after_micros(time_us.into()).await;
        // }
    }

    async fn draw_row(&mut self, row: u8, iter: u8) {
        let time_start: Instant = Instant::now();

        let mut offs: usize = 0;
        let comp_mask = 1 << iter;

        for col in 0..PIN_COUNT {

            if col == row.into() {
                offs +=1;
                continue;
            }

            // if self.buf[row as usize * (PIN_COUNT -1) + col + offs] & comp_mask != 0 {
            if *self.get(row,col + offs) & comp_mask != 0 {
                self.pin_state[col] = STATE_HIGH;
            }
            else {
                self.pin_state[col] = STATE_TRI;
            }

            self.pin_state[row as usize] = STATE_LOW;
        }
        self.latch();

        let pre_latch: Duration = Instant::now() - time_start;

        // https://www.youtube.com/watch?v=8wMKw4m6-Rc&t=452s
        // Thanks, bitluni!
        let delay_mod = 3_u32.pow(u32::from(iter));

        // info!("value: {:#010b}, compmask: {:#010b}, iter: {},  delay: {}",
        //     self.buf[0],comp_mask,
        //     iter, delay_mod);
        self.delay(DELAY_BASE_US * delay_mod).await;

        for col in 0..PIN_COUNT {
            self.pin_state[col] = STATE_TRI;
        }
        self.latch();

        let post_latch: Duration = Instant::now() - time_start;
        // info!("del {}, pre {}, post {} us",
        //     DELAY_BASE_US * delay_mod,
        //     pre_latch.as_micros(),
        //     post_latch.as_micros());

        // Timer::after_millis(50).await;

    }

    fn latch(&mut self){
        for col in 0..PIN_COUNT {
            match self.pin_state[col] {
                STATE_LOW => {
                    self.pin_list[col].set_low();
                }
                STATE_HIGH => {
                    self.pin_list[col].set_high();
                }
                _ => {}
            }
        }

        for col in 0..PIN_COUNT {
            match self.pin_state[col] {
                STATE_LOW => {
                    self.pin_list[col].set_as_output(py32_hal::gpio::Speed::VeryHigh);
                    // let test = self.pin_list[col].get_port();

                    // let b = self.pin_list[col].pin.block();
                }
                STATE_HIGH => {
                    self.pin_list[col].set_as_output(py32_hal::gpio::Speed::VeryHigh);
                }
                _ => {
                    self.pin_list[col].set_as_input(py32_hal::gpio::Pull::None);
                }
            }
        }

    }

    //TODO: https://docs.rs/drawille-nostd/latest/drawille_nostd/
    pub async fn draw(&mut self) {
        // for iter in 0.. 2_u8.pow(BIT_DEPTH.into()) {
        for iter in 0.. BIT_DEPTH {
            for i in 0..PIN_COUNT {
                self.draw_row(i as u8, iter).await;
            }
        }
    }

    #[allow(dead_code)]
    pub fn draw_random(&mut self) {
        for i in 0..PIN_COUNT {
            set_random(&mut self.pin_list[i]);
        }
    }
}        


use py32_hal::{gpio::{Flex, AnyPin}, Peripheral};

use embassy_time::Timer;

use crate::utils;

// awful i know
const STATE_LOW: u8 = 0;
const STATE_HIGH: u8 = 1;
const STATE_TRI: u8 = 2;

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

            let mut c = Charlie {
                pin_list: gpio_list,
                pin_state: [0; PIN_COUNT],
                buf: [0; PIN_COUNT * (PIN_COUNT-1)]
            };

            c.buf = [0; PIN_COUNT * (PIN_COUNT-1)];
            return c
        }
    }

    async fn draw_row(&mut self, row: u8){
        let mut offs: usize = 0;
        for col in 0..PIN_COUNT {

            if col == row.into() {
                offs +=1;
                continue;
            }

            if self.buf[row as usize * (PIN_COUNT -1) + col + offs] > 0 {
                self.pin_state[col] = STATE_HIGH;
            }
            else {
                self.pin_state[col] = STATE_TRI;
            }

            self.pin_state[row as usize] = STATE_LOW;
        }
        self.latch();
        // Timer::after_millis(20).await;
        cortex_m::asm::delay(2000);

        for col in 0..PIN_COUNT {
            self.pin_state[col] = STATE_TRI;
        }
        self.latch();
        // Timer::after_millis(50).await;

    }

    fn latch(&mut self){

        // for col in 0..PIN_COUNT {
        //     self.pin_list[col].yolo(self.pin_state[col]);
        //     // cortex_m::asm::delay(8_000 * 50);
        // }


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

    pub async fn draw(&mut self) {
        for i in 0..PIN_COUNT {
            self.draw_row(i as u8).await;
        }
    }

    #[allow(dead_code)]
    pub fn draw_random(&mut self) {
        for i in 0..PIN_COUNT {
            set_random(&mut self.pin_list[i]);
        }
    }
}        

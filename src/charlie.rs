
use py32_hal::{gpio::{Flex, AnyPin}, Peripheral};

use embassy_time::Timer;

use crate::utils;

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
                self.pin_list[col].set_as_output(py32_hal::gpio::Speed::Low);
                self.pin_list[col].set_high();
            }
            else {
                self.pin_list[col].set_as_input(py32_hal::gpio::Pull::None);
            }

            self.pin_list[row as usize].set_as_output(py32_hal::gpio::Speed::Low);
            self.pin_list[row as usize].set_low();
        }

        Timer::after_millis(40).await;

        for col in 0..PIN_COUNT {
            self.pin_list[col].set_as_output(py32_hal::gpio::Speed::Low);
            self.pin_list[col].set_high();
        }
    }

    pub async fn draw(&mut self) {
        for i in 0..PIN_COUNT {
            self.draw_row(i as u8).await;
        }
    }

    pub fn draw_random(&mut self) {
        for i in 0..PIN_COUNT {
            set_random(&mut self.pin_list[i]);
        }
    }
}        

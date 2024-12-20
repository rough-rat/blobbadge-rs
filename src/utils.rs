use py32_hal::gpio::Flex;

pub fn bootleg_random() -> u32 {
    const M:u32 = 65537;
    const A:u32 = 75;
    const C:u32 = 74;

    static mut RAND: u32 = 0;
    
    unsafe {
        RAND = (RAND * A + C) % M;
        RAND
    }
}

pub fn set_random(pin: &mut Flex<'_>) {
    match bootleg_random() % 3 {
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
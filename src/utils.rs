
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



pub fn bootleg_random_u8() -> u8 {
    const M:u32 = 65537;
    const A:u32 = 75;
    const C:u32 = 74;

    static mut RAND: u32 = 0;
    
    unsafe {
        RAND = (RAND * A + C) % M;
        RAND as u8
    }
}
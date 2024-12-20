
use embassy_time::Timer;

pub async fn charlie_simple_loop<const S: usize>(pin_list: [AnyPin; S]) {
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
                set_random(p);
                Timer::after_millis((0 + utils::bootleg_random() % 30).into()).await; 
            }
            // Timer::after_millis(500).await;  
        }
    }
}

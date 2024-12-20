
use defmt::*;
use embassy_time::Timer;
use py32_hal::adc::{Adc, SampleTime};
use py32_hal::peripherals::ADC;
use py32_hal::{adc, bind_interrupts};

bind_interrupts!(struct Irqs {
    ADC_COMP => adc::InterruptHandler<ADC>;
});

#[embassy_executor::task]
pub async fn run_bat_monitor(p_adc: ADC) {
    let mut adc = Adc::new(p_adc, Irqs);
    adc.set_sample_time(SampleTime::CYCLES71_5);

    let mut vrefint = adc.enable_vref();

    loop {
        let vrefint_sample: u32 = adc.read(&mut vrefint).await.into();
        let vcc: u32 = 1200 * 4095 / vrefint_sample;
        info!("--> {} - {} mV", vrefint_sample, vcc);
        Timer::after_millis(2000).await;
    }
}
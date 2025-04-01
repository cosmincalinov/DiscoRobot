#![no_std]
#![no_main]

use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, uart};
use embassy_rp::gpio::{Level, Output, SlewRate, AnyPin, Flex, Input, Pull};
use embassy_rp::peripherals::{DMA_CH0, PIO0};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_time::{Duration, Timer, Ticker};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};
use portable_atomic::AtomicU16;
use core::sync::atomic::Ordering;
use hcsr04_async::{Config, DistanceUnit, Hcsr04, TemperatureUnit};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
});

#[embassy_executor::task]
async fn cyw43_task(runner: cyw43::Runner<'static, Output<'static>, PioSpi<'static, PIO0, 0, DMA_CH0>>) -> ! {
    runner.run().await
}

fn servo_on_time(pos: f32) -> Duration {
    let servo_min = 500.0;
    let servo_max = 2200.0;

    let servo_us = pos * (servo_max - servo_min) + servo_min;

    Duration::from_micros(servo_us as u64)
}

static SERVO_CONTROL: AtomicU16 = AtomicU16::new(0);

#[embassy_executor::task]
async fn servo_control(mut pin: Output<'static>, control: &'static AtomicU16) {
    let mut ticker = Ticker::every(Duration::from_hz(100));
    loop {
        ticker.next().await;
        pin.set_high();

        let val = control.load(Ordering::Relaxed);

        Timer::after(servo_on_time(val as f32 / u16::MAX as f32)).await;
        pin.set_low();
    }        
}


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_rp::init(Default::default());
    let fw = include_bytes!("../../../../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../../../cyw43-firmware/43439A0_clm.bin");

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0, Irqs);
    let spi = PioSpi::new(&mut pio.common, pio.sm0, pio.irq0, cs, p.PIN_24, p.PIN_29, p.DMA_CH0);

    static STATE: StaticCell<cyw43::State> = StaticCell::new();
    let state = STATE.init(cyw43::State::new());
    let (_net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(cyw43_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    let mut servo = Output::new(p.PIN_15, Level::Low);
    servo.set_slew_rate(SlewRate::Slow);

    let trigger = Output::new(p.PIN_26, Level::Low);
    let echo = Input::new(p.PIN_22, Pull::None);
    
    let config = Config {
        distance_unit: DistanceUnit::Centimeters,
        temperature_unit: TemperatureUnit::Celsius,
    };

    let mut sensor = Hcsr04::new(trigger, echo, config);
    let temperature = 24.0;

    spawner.must_spawn(servo_control(servo, &SERVO_CONTROL));

    control.gpio_set(0, true).await;
    info!("Hello, world!");

    loop {
        let distance = sensor.measure(temperature).await;
        match distance {
            Ok(distance) => info!("Distance: {} cm", distance),
            Err(e) => info!("Error: {:?}", e),
        }
        Timer::after(Duration::from_secs(1)).await;

        // SERVO_CONTROL.store((1.0 * u16::MAX as f32) as u16, Ordering::Relaxed);
        // Timer::after(Duration::from_millis(1000)).await;
        // SERVO_CONTROL.store((0.0 * u16::MAX as f32) as u16, Ordering::Relaxed);
    }
}

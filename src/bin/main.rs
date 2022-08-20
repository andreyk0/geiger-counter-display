#![deny(unsafe_code)]
#![no_main]
#![no_std]

use fugit::RateExtU32;
use panic_rtt_target as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::gpio::PinState;
use stm32f1xx_hal::prelude::*;
use systick_monotonic::{fugit::Duration, Systick};

use geiger_counter_display::consts::*;
use geiger_counter_display::types::*;

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use stm32f1xx_hal::i2c::blocking::BlockingI2c;

#[app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        led: LedPin,
        state: bool,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MonoTimer = Systick<1000>;

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut flash = cx.device.FLASH.constrain();
        let rcc = cx.device.RCC.constrain();
        let mono = Systick::new(cx.core.SYST, SYS_FREQ_HZ);

        rtt_init_print!();
        rprintln!("init");

        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(SYS_FREQ)
            .pclk1(36.MHz())
            .freeze(&mut flash.acr);

        let mut afio = cx.device.AFIO.constrain();

        cx.core.DWT.enable_cycle_counter(); // Needed by BlockingI2c

        // Setup LED
        let mut gpioc = cx.device.GPIOC.split();
        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::Low);

        // Display I2C
        let mut gpiob = cx.device.GPIOB.split();
        let scl = gpiob.pb6.into_alternate_open_drain(&mut gpiob.crl);
        let sda = gpiob.pb7.into_alternate_open_drain(&mut gpiob.crl);

        let i2c_bus = BlockingI2c::i2c1(
            cx.device.I2C1,
            (scl, sda),
            &mut afio.mapr,
            stm32f1xx_hal::i2c::Mode::standard(100.kHz()),
            clocks,
            10000,
            10,
            10000,
            10000,
        );

        let interface = I2CDisplayInterface::new(i2c_bus);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().unwrap();

        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_10X20)
            .text_color(BinaryColor::On)
            .build();

        Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.flush().unwrap();

        // Schedule the blinking task
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();

        (
            Shared {},
            Local { led, state: false },
            init::Monotonics(mono),
        )
    }

    #[task(local = [led, state])]
    fn blink(cx: blink::Context) {
        rprintln!("blink");
        if *cx.local.state {
            cx.local.led.set_high();
            *cx.local.state = false;
        } else {
            cx.local.led.set_low();
            *cx.local.state = true;
        }
        blink::spawn_after(Duration::<u64, 1, 1000>::from_ticks(1000)).unwrap();
    }
}

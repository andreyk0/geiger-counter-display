#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;

use fugit::RateExtU32;
use rtic::app;
use stm32f1xx_hal::gpio::PinState;
use stm32f1xx_hal::prelude::*;
use systick_monotonic::Systick;

use cortex_m::asm::delay;

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use stm32f1xx_hal::i2c::blocking::BlockingI2c;

use geiger_counter_display::consts::*;
use geiger_counter_display::display::*;
use geiger_counter_display::pulse::*;
use geiger_counter_display::timer::*;
use geiger_counter_display::types::*;

#[app(device = stm32f1xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {

    use super::*;

    #[shared]
    struct Shared {
        pulse_timer: PulseTimer,
        samples: SampleBuffer,
    }

    #[local]
    struct Local {
        led: LedPin,
        lcd: LcdDisplay,
    }

    #[monotonic(binds = SysTick, default = true)]
    type MyMono = Systick<100>; // 100 Hz / 10 ms granularity

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut flash = cx.device.FLASH.constrain();

        PulseTimer::enable(&mut cx.device.RCC);

        let rcc = cx.device.RCC.constrain();
        let mono = Systick::new(cx.core.SYST, SYS_FREQ_HZ);

        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(SYS_FREQ)
            .pclk1(36.MHz())
            .pclk2(4_500_000.Hz()) // lower frequency to use timer hardware to filter out longer glitches
            .freeze(&mut flash.acr);

        let mut afio = cx.device.AFIO.constrain();

        cx.core.DWT.enable_cycle_counter(); // Needed by BlockingI2c

        let mut gpioa = cx.device.GPIOA.split();
        let mut gpiob = cx.device.GPIOB.split();
        let mut gpioc = cx.device.GPIOC.split();

        // Setup LED
        let led = gpioc
            .pc13
            .into_push_pull_output_with_state(&mut gpioc.crh, PinState::Low);

        // Display I2C
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
        let mut lcd = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        lcd.init().unwrap();

        // Set up pulse timer
        gpioa.pa8.into_floating_input(&mut gpioa.crh); // capture geiger pulse
        let pulse_timer = PulseTimer::new(cx.device.TIM1, cx.device.TIM2);

        // Schedule the blinking task
        blink::spawn_at(monotonics::now() + 1.secs()).unwrap();

        (
            Shared {
                pulse_timer,
                samples: SampleBuffer::new(),
            },
            Local { led, lcd },
            init::Monotonics(mono),
        )
    }

    #[idle(local = [lcd], shared = [samples, pulse_timer])]
    fn idle(mut cx: idle::Context) -> ! {
        loop {
            let ts_from = monotonics::now() - 10.secs();
            let (slast, savg) = cx.shared.samples.lock(|s| s.get(ts_from));

            cx.local.lcd.clear();
            render_output(cx.local.lcd, slast, savg).unwrap();
            cx.local.lcd.flush().unwrap();

            delay(SYS_FREQ_HZ / 4);
        }
    }

    #[task(priority = 2, binds = TIM1_CC, shared = [pulse_timer, samples])]
    fn tim1cc(mut cx: tim1cc::Context) {
        let s = cx.shared.pulse_timer.lock(|pt| pt.poll());
        cx.shared
            .samples
            .lock(|ls| s.map(|x| ls.add(PulseSample::new(x, monotonics::now()))));
    }

    #[task(local = [led])]
    fn blink(cx: blink::Context) {
        cx.local.led.toggle();
        blink::spawn_at(monotonics::now() + 1.secs()).unwrap();
    }
}

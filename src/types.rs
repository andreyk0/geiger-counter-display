use stm32f1xx_hal::gpio::*;

use ssd1306::{prelude::*, Ssd1306};

pub type LedPin = gpioc::PC13<Output<PushPull>>;
use stm32f1xx_hal::i2c::blocking::BlockingI2c;

pub type LcdDisplay = Ssd1306<
    I2CInterface<
        BlockingI2c<
            stm32f1xx_hal::pac::I2C1,
            (
                Pin<Alternate<OpenDrain>, CRL, 'B', 6>,
                Pin<Alternate<OpenDrain>, CRL, 'B', 7>,
            ),
        >,
    >,
    DisplaySize128x64,
    ssd1306::mode::BufferedGraphicsMode<DisplaySize128x64>,
>;

use stm32f1xx_hal::stm32::{RCC, TIM1};

pub struct PulseTimer {
    timer: TIM1,
}

impl PulseTimer {
    pub fn enable(rcc: &mut RCC) -> () {
        rcc.apb2enr.modify(|_, w| w.tim1en().set_bit()); // enable clock
    }

    pub fn new(mut timer: TIM1) -> Self {
        setup_pulse_timer(&mut timer);

        PulseTimer { timer }
    }

    pub fn poll(&mut self) -> Option<u16> {
        let duration = self.timer.ccr1.read().bits() as u16;
        let overcapture = self.timer.sr.read().cc1of().bit_is_set();
        self.timer.sr.modify(|_, w| w.cc1of().clear_bit());
        if overcapture {
            None
        } else {
            Some(duration)
        }
    }
}

// RM0008 14.3.6 Input capture mode
fn setup_pulse_timer(tim: &mut TIM1) {
    // RM0008 14.4.7 TIM1 and TIM8 capture/compare mode register 1 (TIMx_CCMR1)
    unsafe {
        tim.ccmr1_input().modify(|_, w| {
            w.cc1s()
                .ti1()
                .ic1f()
                .bits(0b1111) // input capture 1 filter
                .ic1psc()
                .bits(0x0)
        });
    }

    // RM0008 15.4.1 TIMx control register 1 (TIMx_CR1)
    tim.cr1.modify(
        |_, w| w.ckd().div4(), // filter clock prescaler
    );

    // RM0008 14.4.9 TIM1 and TIM8 capture/compare enable register (TIMx_CCER)
    tim.ccer.modify(
        |_, w| {
            w.cc1p().clear_bit(). // positive edge
        cc1e().set_bit()
        }, // enable capture
    );

    tim.dier.modify(
        |_, w| w.cc1ie().set_bit(), // enable interrupt
    );
}

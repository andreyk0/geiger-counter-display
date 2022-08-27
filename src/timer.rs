use stm32f1xx_hal::stm32::{RCC, TIM1, TIM2};

use crate::consts::*;

pub struct PulseTimer {
    timer1: TIM1,
    timer2: TIM2,
    last_pulse_ts: u32,
}

impl PulseTimer {
    pub fn enable(rcc: &mut RCC) -> () {
        rcc.apb2enr.modify(|_, w| w.tim1en().set_bit()); // enable clock
        rcc.apb1enr.modify(|_, w| w.tim2en().set_bit()); // enable clock
    }

    pub fn new(mut timer1: TIM1, mut timer2: TIM2) -> Self {
        setup_pulse_timer(&mut timer1);
        setup_slave_timer(&mut timer2);

        PulseTimer {
            timer1,
            timer2,
            last_pulse_ts: 0u32,
        }
    }

    // last period, seconds
    pub fn poll(&mut self) -> Option<f32> {
        let ts = self.get_time();
        let overcapture = self.timer1.sr.read().cc1of().bit_is_set();

        /*if overcapture {
            hprintln!("ts: {}, oc: {}", ts, overcapture);
        }*/

        let diff = (ts.wrapping_sub(self.last_pulse_ts) as f32) / TIM_TICKS_PER_SEC;

        // some glitches are not filtered by the timer hardware
        let result = if diff > PULSE_MIN_PERIOD_SEC {
            self.last_pulse_ts = ts;
            Some(diff)
        } else {
            None
        };

        // CC1IF is normally cleared by reading the captured value but
        //       self.timer1.ccr1.read().bits() as u16;
        // we ignore that to read 2 16-bit values from tim1,2
        self.timer1
            .sr
            .modify(|_, w| w.cc1of().clear().cc1if().clear());

        if overcapture {
            None
        } else {
            result
        }
    }

    fn get_time(&self) -> u32 {
        let th1 = self.timer2.cnt.read().bits() as u16;
        let tl1 = self.timer1.cnt.read().bits() as u16;
        let th2 = self.timer2.cnt.read().bits() as u16;

        // hack around possible rollover
        if th2 == th1 {
            (th2 as u32) << 16 | tl1 as u32
        } else {
            let tl2 = self.timer1.cnt.read().bits() as u16;
            (th2 as u32) << 16 | tl2 as u32
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
                .bits(0b1111) // 1111: fSAMPLING=fDTS/32, N=8 // input capture 1 filter
                .ic1psc()
                .bits(0x0)
        });
    }

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

    // Enable master mode
    tim.cr2.modify(|_, w| w.mms().update());

    // RM0008 15.4.1 TIMx control register 1 (TIMx_CR1)
    tim.cr1.modify(|_, w| {
        w.ckd()
            .div4() // 10: tDTS=4*tCK_INT // filter clock prescaler
            .cen()
            .enabled() // enable counter
    });
}

// To increase measured time periods between pulses
// RM0008 15.3.15 Timer synchronization
fn setup_slave_timer(tim: &mut TIM2) {
    // Enable slave mode
    tim.smcr.modify(|_, w| w.ts().itr0().sms().ext_clock_mode());

    tim.cr1.modify(|_, w| {
        w.cen().enabled() // enable counter
    });
}

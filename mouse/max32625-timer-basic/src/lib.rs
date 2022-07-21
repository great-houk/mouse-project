#![no_std]

pub use timer::{Timer, TimerError};

macro_rules! impl_for_raw_timers {
    ($name:ident / $enum:ty: $($code:tt)*) => {
        mod __tmr0 {
            use super::*;
            #[allow(dead_code)]
            const TMR: $enum = <$enum>::TMR0;
            impl $name for max32625::TMR0 {
                $($code)*
            }
        }
        mod __tmr1 {
            use super::*;
            #[allow(dead_code)]
            const TMR: $enum = <$enum>::TMR1;
            impl $name for max32625::TMR1 {
                $($code)*
            }
        }
        mod __tmr2 {
            use super::*;
            #[allow(dead_code)]
            const TMR: $enum = <$enum>::TMR2;
            impl $name for max32625::TMR2 {
                $($code)*
            }
        }
        mod __tmr3 {
            use super::*;
            #[allow(dead_code)]
            const TMR: $enum = <$enum>::TMR3;
            impl $name for max32625::TMR3 {
                $($code)*
            }
        }
        mod __tmr4 {
            use super::*;
            #[allow(dead_code)]
            const TMR: $enum = <$enum>::TMR4;
            impl $name for max32625::TMR4 {
                $($code)*
            }
        }
        mod __tmr5 {
            use super::*;
            #[allow(dead_code)]
            const TMR: $enum = <$enum>::TMR5;
            impl $name for max32625::TMR5 {
                $($code)*
            }
        }
    };
}

mod timer;

#![no_std]

pub use driver::{Pmw3389Driver, Pmw3389Error};
pub use pmw3389::{MotionReport, Pmw3389, Register};

mod driver;
mod pmw3389;

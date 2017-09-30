#![feature(core_intrinsics)]
#![no_std]

mod gpio;
mod register;

use gpio::Gpio;

pub struct Pi {
    gpio: Gpio,
}

impl Pi {
    pub fn new() -> Self {
        Pi{
            gpio: Gpio::new(),
        }
    }
}

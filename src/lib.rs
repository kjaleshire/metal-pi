#![feature(core_intrinsics, lang_items)]
#![no_std]

mod gpio;

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

#[lang = "eh_personality"]
extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt() -> ! {
    loop {}
}

#![feature(core_intrinsics, lang_items)]
#![no_std]

mod gpio;

use gpio::Gpio;

struct Pi {
    gpio: Gpio,
}

impl Pi {
    fn new() -> Self {
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

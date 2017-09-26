use core::intrinsics::{volatile_store, volatile_load};
use core::mem;

pub enum Error {
    OutOfRange(&'static str)
}

pub enum PinFunction {
    Input = 0b000,
    Output = 0b001,
    AltFn0 = 0b100,
    AltFn1 = 0b101,
    AltFn2 = 0b110,
    AltFn3 = 0b111,
    AltFn4 = 0b011,
    AltFn5 = 0b010,
}

pub struct Gpio {
    gpio_base: usize,

    pin_count: usize,
    fsel_registers: usize,
    set_registers: usize,
    clr_registers: usize,
}

impl Gpio {
    pub fn new() -> Self {
        let gpio_address_base = 0x3F00_0000;

        Self {
            gpio_base: gpio_address_base,

            pin_count: 59,
            fsel_registers: 6,
            set_registers: 2,
            clr_registers: 2,
        }
    }

    pub fn select_function(&self, pin: usize, function: PinFunction)
        -> Result<(), Error> {
        if pin > self.pin_count {
            return Err(Error::OutOfRange("Pin value out of range"));
        }

        let register_num = pin / self.fsel_registers;
        let fsel_register_offset = register_num * mem::size_of::<usize>();
        let register = FselRegister::new(self.gpio_base, fsel_register_offset);

        let offset = pin % self.fsel_registers;
        register.select_function(offset, function);
        Ok(())
    }
}

trait Register {
    fn address(&self) -> usize;

    fn write(&self, value: usize) {
        unsafe {
            volatile_store::<usize>(self.address() as *mut usize, value);
        }
    }

    fn read(&self) -> usize {
        unsafe {
            volatile_load::<usize>(self.address() as *mut usize)
        }
    }
}

struct FselRegister {
    address: usize
}

impl FselRegister {
    fn new(gpio_base: usize, fsel_register_offset: usize) -> Self {
        let gpio_fsel_offset = 0x20_0000;

        let address = gpio_base + gpio_fsel_offset + fsel_register_offset;
        assert!(address & gpio_base == gpio_base);

        Self{address: address}
    }

    fn select_function(&self, pin: usize, function: PinFunction) {
        let current_value = self.read();

        let shift = pin * 3;

        let new_value = function as usize >> shift;

        self.write(current_value | new_value);
    }
}

impl Register for FselRegister {
    fn address(&self) -> usize {
        self.address
    }
}

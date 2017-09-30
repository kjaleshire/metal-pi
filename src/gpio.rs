use core::intrinsics::{volatile_store, volatile_load};
use core::mem;

use register::{Register, RegisterSetting};

pub enum Error {
    OutOfRange(&'static str)
}

pub enum PinFunction {
    Input =  0b000,
    Output = 0b001,
    AltFn0 = 0b100,
    AltFn1 = 0b101,
    AltFn2 = 0b110,
    AltFn3 = 0b111,
    AltFn4 = 0b011,
    AltFn5 = 0b010,
}

impl RegisterSetting for PinFunction {
    fn value(&self) -> usize { self as usize }
    fn mask(&self) -> usize { 0b111 }
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
        let register_num = pin / self.fsel_registers;
        let fsel_register_offset = register_num * mem::size_of::<usize>();
        let register = FselRegister::new(self.gpio_base, fsel_register_offset);

        let offset = pin % self.fsel_registers;
        register.select_function(offset, function);
        Ok(())
    }

    pub fn set_pin(&self, pin: usize) {
        let register_num = pin / self.set_registers;
        let set_register_offset = register_num * mem::size_of::<usize>();
        let register = SetRegister::new(self.gpio_base, set_register_offset);

        let offset = pin % self.set_registers;
        register.set_pin(offset);
        Ok(())
    }

    pub fn clear_pin(&self, pin: usize) -> bool {
        let register_num = pin / self.clr_registers;
        let clr_register_offset = register_num * mem::size_of::<usize>();
        let register = ClrRegister::new(self.gpio_base, clr_register_offset);

        let offset = pin % self.clr_registers;
        register.clear_pin(offset);
        Ok(())
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
        let shift = pin * 3;
        self.write_value_at_offset(function, shift);
    }
}

impl Register for FselRegister {
    fn address(&self) -> usize {
        self.address
    }
}

struct BitSetting {}

impl RegisterSetting for BitSetting {
    fn value(&self) -> usize { 0b1 }
    fn mask(&self) -> usize { 0b1 }
}

struct SetRegister {
    address: usize
}

impl SetRegister {
    fn new(gpio_base: usize, set_register_offset: usize) -> Self {
        let gpio_set_offset = 0x20_001C;

        let address = gpio_base + gpio_set_offset + set_register_offset;
        assert!(address & gpio_base == gpio_base);

        Self{address: address}
    }

    fn set_pin(&self, pin: usize) {
        self.write_value_at_offset(BitSetting{}, pin);
    }
}

impl Register for SetRegister {
    fn address(&self) -> usize {
        self.address
    }
}

struct ClrRegister {
    address: usize
}

impl ClrRegister {
    fn new(gpio_base: usize, clr_register_offset: usize) -> Self {
        let gpio_clr_offset = 0x20_0028;

        let address = gpio_base + gpio_clr_offset + clr_register_offset;
        assert!(address & gpio_base == gpio_base);

        Self{address: address}
    }

    fn clear_pin(&self, pin: usize) {
        self.write_value_at_offset(BitSetting{}, pin);
    }
}

impl Register for ClrRegister {
    fn address(&self) -> usize {
        self.address
    }
}

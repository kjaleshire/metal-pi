use register::{Register, RegisterSetting};

#[derive(Copy, Clone)]
pub enum PinFn {
    Input =  0b000,
    Output = 0b001,
    AltFn0 = 0b100,
    AltFn1 = 0b101,
    AltFn2 = 0b110,
    AltFn3 = 0b111,
    AltFn4 = 0b011,
    AltFn5 = 0b010,
}

impl RegisterSetting for PinFn {
    fn value(&self) -> usize { *self as usize }
    fn mask(&self) -> usize { 0b111 }
}

pub fn gpio_address_base() -> usize { 0x3F00_0000 }
pub fn fsel_registers() -> usize { 6 }
pub fn set_registers() -> usize { 2 }
pub fn clr_registers() -> usize { 2 }

pub fn select_fn(pin: usize, function: PinFn) {
    let register_num = pin / FselRegister::pins_per_register();
    let register = FselRegister::new(gpio_address_base(), register_num);

    let offset = pin % FselRegister::pins_per_register();
    register.select_fn(offset, function);
}

pub fn set_pin(pin: usize) {
    let register_num = pin / SetRegister::pins_per_register();
    let register = SetRegister::new(gpio_address_base(), register_num);

    let offset = pin % SetRegister::pins_per_register();
    register.set_pin(offset);
}

pub fn clear_pin(pin: usize) {
    let register_num = pin / ClrRegister::pins_per_register();
    let register = ClrRegister::new(gpio_address_base(), register_num);

    let offset = pin % ClrRegister::pins_per_register();
    register.clear_pin(offset);
}


pub struct FselRegister {
    address: usize
}

impl FselRegister {
    pub fn new(gpio_base: usize, register_number: usize) -> Self {
        let address = gpio_base + Self::bank_offset() + Self::register_offset(register_number);
        assert!(address & gpio_base == gpio_base);

        Self{address: address}
    }

    pub fn select_fn(&self, pin: usize, function: PinFn) {
        let shift = pin * 3;
        self.write_value_at_offset(function, shift);
    }
}

impl Register for FselRegister {
    fn address(&self) -> usize {
        self.address
    }

    fn pins_per_register() -> usize { 10 }

    fn bank_offset() -> usize { 0x20_0000 }
}

pub struct BitSetting {}

impl RegisterSetting for BitSetting {
    fn value(&self) -> usize { 0b1 }
    fn mask(&self) -> usize { 0b1 }
}

pub struct SetRegister {
    address: usize
}

impl SetRegister {
    pub fn new(gpio_base: usize, register_number: usize) -> Self {
        let address = gpio_base + Self::bank_offset() + Self::register_offset(register_number);
        assert!(address & gpio_base == gpio_base);

        Self{address: address}
    }

    pub fn set_pin(&self, pin: usize) {
        self.write(0b1 << pin);
    }
}

impl Register for SetRegister {
    fn address(&self) -> usize {
        self.address
    }

    fn pins_per_register() -> usize { 32 }

    fn bank_offset() -> usize { 0x20_001C }
}

struct ClrRegister {
    address: usize
}

impl ClrRegister {
    pub fn new(gpio_base: usize, register_number: usize) -> Self {
        let address = gpio_base + Self::bank_offset() + Self::register_offset(register_number);
        assert!(address & gpio_base == gpio_base);

        Self{address: address}
    }

    pub fn clear_pin(&self, pin: usize) {
        self.write(0b1 << pin);
    }
}

impl Register for ClrRegister {
    fn address(&self) -> usize {
        self.address
    }

    fn pins_per_register() -> usize { 32 }

    fn bank_offset() -> usize { 0x20_0028 }
}

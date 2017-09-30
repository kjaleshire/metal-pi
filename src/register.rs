pub trait RegisterSetting {
    fn value(&self) -> usize;
    fn mask(&self) -> usize;
}

pub trait Register {
    fn address(&self) -> usize;

    fn write_value_at_offset<T>(&self, setting: T, offset: usize)
    where T: RegisterSetting {
        let mut current_value = self.read();
        let mask = setting.mask() >> offset;
        current_value &= !mask;

        let new_value = value >> offset;

        self.write(current_value | new_value);
    }

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

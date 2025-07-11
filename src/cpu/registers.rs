pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagRegister,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn get_bc(&self) -> u16 {
        ((self.b as u16) << 8) | (self.c as u16)
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }
}

pub struct FlagRegister {
    pub zero: bool,
    pub substract: bool,
    pub half_carry: bool,
    pub carry: bool,
}

impl std::convert::From<FlagRegister> for u8 {
    fn from(flag: FlagRegister) -> Self {
        ((flag.zero as u8) << 7) |
        ((flag.substract as u8) << 6) |
        ((flag.half_carry as u8) << 5) |
        ((flag.carry as u8) << 4)
    }
}

impl std::convert::From<u8> for FlagRegister {
    fn from(value: u8) -> Self {
        let zero = ((value >> 7) & 1) == 1;
        let substract = ((value >> 6) & 1) == 1;
        let half_carry = ((value >> 5) & 1) == 1;
        let carry = ((value >> 4) & 1) == 1;
        Self {
            zero,
            substract,
            half_carry,
            carry,
        }
    }
}
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
    pub pc: u16,
    pub sp: u16
}

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum FlagMask {
    Z = 0x80,
    N = 0x40,
    H = 0x20,
    C = 0x10,
}

impl Registers {

    pub fn new() -> Registers {
        Registers {
            a: 0x01,
            f: 0xB0,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            pc: 0x0100,
            sp: 0xFFFE
        }
    }

    pub fn set_flag_val(&mut self, flag: FlagMask, val: bool) {
        self.f &= !(flag as u8);
        let a: u8 = if val {0xFF} else {0x00};
        self.f |= a & (flag as u8);
    }

    pub fn set_flag(&mut self, flag: FlagMask) {
        self.f |= flag as u8;
    }

    pub fn clear_flag(&mut self, flag: FlagMask) {
        self.f &= !(flag as u8);
    }

    pub fn get_flag(&self, flag: FlagMask) -> bool {
        (self.f & flag as u8) > 0
    }

    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = (val & 0xF0) as u8;
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f as u16
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

}

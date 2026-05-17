use crate::cpu::decode::extract_bits;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OperationMode {
    Usr = 0b10000,
    Fiq = 0b10001,
    Irq = 0b10010,
    Svc = 0b10011,
    Abt = 0b10111,
    Und = 0b11011,
    Sys = 0b11111
}

impl From<u32> for OperationMode {
    fn from(value: u32) -> Self {
        match value {
            0b10000 => OperationMode::Usr,
            0b10001 => OperationMode::Fiq,
            0b10010 => OperationMode::Irq,
            0b10011 => OperationMode::Svc,
            0b10111 => OperationMode::Abt,
            0b11011 => OperationMode::Und,
            0b11111 => OperationMode::Sys,
            _ => panic!("Invalid operation mode")
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IsaMode {
    Arm,
    Thumb
}

impl From<bool> for IsaMode {
    fn from(value: bool) -> Self {
        match value {
            false => IsaMode::Arm,
            true => IsaMode::Thumb,
        }
    }
}

impl From<IsaMode> for bool {
    fn from(mode: IsaMode) -> bool {
        match mode {
            IsaMode::Arm => false,
            IsaMode::Thumb => true,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cpsr {
    pub n: bool,
    pub z: bool,
    pub c: bool,
    pub v: bool,
    pub i: bool,
    pub f: bool,
    pub t: IsaMode,
    pub mode: OperationMode
}

impl From<u32> for Cpsr {
    fn from(value: u32) -> Self {
        Cpsr {
            n: extract_bits(value, 31, 1) == 1,
            z: extract_bits(value, 30, 1) == 1,
            c: extract_bits(value, 29, 1) == 1,
            v: extract_bits(value, 28, 1) == 1,
            i: extract_bits(value, 7, 1) == 1,
            f: extract_bits(value, 6, 1) == 1,
            t: (extract_bits(value, 5, 1) == 1).into(),
            mode: extract_bits(value, 4, 5).into()
        }
    }
}

impl From<Cpsr> for u32 {
    fn from(cpsr: Cpsr) -> u32 {
        let operation_mode = cpsr.mode as u32;
        let cpu_mode = bool::from(cpsr.t) as u32;
        (cpsr.n as u32) << 31
            | (cpsr.z as u32) << 30
            | (cpsr.c as u32) << 29
            | (cpsr.v as u32) << 28
            | (cpsr.i as u32) << 7
            | (cpsr.f as u32) << 6
            | cpu_mode << 5
            | operation_mode
    }
}

impl Default for Cpsr {
    fn default() -> Self {
        Cpsr {
            n: false,
            z: false,
            c: false,
            v: false,
            i: true,
            f: true,
            t: IsaMode::Arm,
            mode: OperationMode::Sys,
        }
    }
}

#[derive(Default)]
pub struct Registers {
    // R0-R12, SP (R13), LR (R14), PC (R15). R0-R7 and PC shared across all modes
    pub gp_regs: [u32; 16],

    // R8-R12 bank for FIQ
    pub r8_r12_fiq: [u32; 5],
    // R8-R12 bank shared by USR/SYS and all non-FIQ modes
    pub r8_r12_usr: [u32; 5],
    
    // Banked SP (R13) and LR (R14) pairs
    pub sp_lr_usr: [u32; 2],
    pub sp_lr_fiq: [u32; 2],
    pub sp_lr_svc: [u32; 2],
    pub sp_lr_abt: [u32; 2],
    pub sp_lr_irq: [u32; 2],
    pub sp_lr_und: [u32; 2],

    // Current Status Register
    pub cpsr: Cpsr,

    // Saved Status Registers
    pub spsr_svc: Cpsr,
    pub spsr_irq: Cpsr,
    pub spsr_fiq: Cpsr,
    pub spsr_und: Cpsr,
    pub spsr_abt: Cpsr
}

impl Registers {

    pub const PC_IDX: usize = 15;

    pub fn read_register(&self, reg: u32) -> u32 {
        self.gp_regs[(reg & 0xF) as usize]
    }

    pub fn write_register(&mut self, reg: u32, val: u32) {
        self.gp_regs[(reg & 0xF) as usize] = val
    }

    pub fn increase_pc(&mut self, incr: u32) {
        self.gp_regs[Self::PC_IDX] += incr;
    }

    pub fn change_mode(&mut self, new_mode: OperationMode) {

        let current_mode = self.cpsr.mode;

        if current_mode == new_mode {
            return;
        }

        self.save_banked_regs();
        self.load_banked_regs(new_mode);

        match new_mode {
            OperationMode::Svc => self.spsr_svc = self.cpsr,
            OperationMode::Irq => self.spsr_irq = self.cpsr,
            OperationMode::Fiq => self.spsr_fiq = self.cpsr,
            OperationMode::Und => self.spsr_und = self.cpsr,
            OperationMode::Abt => self.spsr_abt = self.cpsr,
            _ => {}
        }
    }

    pub fn save_banked_regs (&mut self) {

        let current_mode = self.cpsr.mode;

        if current_mode == OperationMode::Fiq {
            self.r8_r12_fiq.copy_from_slice(&self.gp_regs[8..13]);
        } else {
            self.r8_r12_usr.copy_from_slice(&self.gp_regs[8..13]);
        }

        let banked_sp_lr = match current_mode {
            OperationMode::Usr | OperationMode::Sys => &mut self.sp_lr_usr,
            OperationMode::Fiq                      => &mut self.sp_lr_fiq,
            OperationMode::Irq                      => &mut self.sp_lr_irq,
            OperationMode::Svc                      => &mut self.sp_lr_svc,
            OperationMode::Abt                      => &mut self.sp_lr_abt,
            OperationMode::Und                      => &mut self.sp_lr_und
        };

        banked_sp_lr.copy_from_slice(&self.gp_regs[13..15]);
    }

    pub fn load_banked_regs(&mut self, mode: OperationMode) {

        if mode == OperationMode::Fiq {
            self.gp_regs[8..13].copy_from_slice(&self.r8_r12_fiq);
        } else {
            self.gp_regs[8..13].copy_from_slice(&self.r8_r12_usr);
        }
        
        let new_sp_lr  = match mode {
            OperationMode::Usr | OperationMode::Sys => &self.sp_lr_usr,
            OperationMode::Fiq                      => &self.sp_lr_fiq,
            OperationMode::Irq                      => &self.sp_lr_irq,
            OperationMode::Svc                      => &self.sp_lr_svc,
            OperationMode::Abt                      => &self.sp_lr_abt,
            OperationMode::Und                      => &self.sp_lr_und
        };

        self.gp_regs[13..15].copy_from_slice(new_sp_lr );
    }

    pub fn get_negative_flag(&self) -> bool {
        self.cpsr.n
    }

    pub fn get_zero_flag(&self) -> bool {
        self.cpsr.z
    }

    pub fn get_carry_flag(&self) -> bool {
        self.cpsr.c
    }

    pub fn get_overflow_flag(&self) -> bool {
        self.cpsr.v
    }

    pub fn set_negative_flag(&mut self, flag: bool) {
        self.cpsr.n = flag
    }

    pub fn set_zero_flag(&mut self, flag: bool) {
        self.cpsr.z = flag
    }

    pub fn set_carry_flag(&mut self, flag: bool) {
        self.cpsr.c = flag
    }

    pub fn set_overflow_flag(&mut self, flag: bool) {
        self.cpsr.v = flag
    }

}

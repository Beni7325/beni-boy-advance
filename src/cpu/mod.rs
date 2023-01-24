
mod registers;
mod instructions;

use crate::mmu::Mmu;
use self::registers::Registers;



enum CpuState {
    Running,
    Halted
}

pub struct Cpu {
    regs: Registers,
    state: CpuState
}

impl Cpu {

    pub fn new() -> Cpu {
        Cpu {
            regs: Registers::new(),
            state: CpuState::Running
        }
    }

    pub fn run_instruction(&mut self, mmu: &mut Mmu) -> u8 {

        match self.state {

            CpuState::Halted => {

            },

            CpuState::Running => {

                /*println!("A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})",
                    self.regs.a, self.regs.f, self.regs.b, self.regs.c, self.regs.d, self.regs.e, self.regs.h, self.regs.l, self.regs.sp, self.regs.pc,
                    mmu.read_byte(self.regs.pc), mmu.read_byte(self.regs.pc.wrapping_add(1)), mmu.read_byte(self.regs.pc.wrapping_add(2)), mmu.read_byte(self.regs.pc.wrapping_add(3))
                );*/

                let instr = mmu.read_byte(self.regs.pc);
                self.regs.pc = self.regs.pc.wrapping_add(1);

                return self.instructions(mmu, instr);
            }

        }

        0
    }

}

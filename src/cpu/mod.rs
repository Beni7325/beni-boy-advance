
mod registers;
mod instructions;
mod interrupts;

pub use self::interrupts::InterruptMask;

use crate::mmu::Mmu;
use self::{registers::Registers, interrupts::InterruptMasterEnable};


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

                // TODO: Implement HALT BUG

                // If there is an interrupt to serve
                if (mmu.interrupt_flag & 0x1F & mmu.interrupt_enable) > 0 {
                    self.state = CpuState::Running;
                    return 0;
                } else {
                    return 1;
                }

            },

            CpuState::Running => {

                if self.regs.ime == InterruptMasterEnable::Enabled {

                    // If there is an interrupt to serve
                    if (mmu.interrupt_flag & 0x1F & mmu.interrupt_enable) > 0 {
                        return self.handle_interrupts(mmu);
                    }
    
                } else if self.regs.ime == InterruptMasterEnable::EnabledWithDelay {
                    self.regs.ime = InterruptMasterEnable::Enabled;
                }

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

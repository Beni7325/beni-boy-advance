use crate::mmu::Mmu;
use super::Cpu;


const INTERR_ADDRESSES: [u16;5] = [0x40, 0x48, 0x50, 0x58, 0x60];

#[derive(PartialEq, Eq)]
pub enum InterruptMasterEnable {
    Disabled,
    Enabled,
    EnabledWithDelay
}

#[repr(u8)]
pub enum InterruptMask {
    Timer = 0x04
}

impl Cpu {
    
    pub fn handle_interrupts(&mut self, mmu: &mut Mmu) -> u8 {

        let mut mask: u8 = 1;

        for addr in INTERR_ADDRESSES {

            if (mmu.interrupt_flag & mmu.interrupt_enable & mask) > 0 {

                self.stack_push(mmu, self.regs.pc);
                self.regs.pc = addr;
                self.regs.ime = InterruptMasterEnable::Disabled;
                mmu.interrupt_flag &= !mask;

                return 5;
            }

            mask <<= 1;
        }

        0
    }

}

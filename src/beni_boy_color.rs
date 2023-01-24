use crate::cpu::Cpu;
use crate::mmu::Mmu;


pub struct BeniBoyColor {
    cpu: Cpu,
    mmu: Mmu
}

impl BeniBoyColor {

    pub fn new(rom_path: &str) -> BeniBoyColor {
        BeniBoyColor { cpu: Cpu::new(), mmu: Mmu::new(rom_path) }
    }

    pub fn run(&mut self, m_cycles: u64) {

        let mut i = 0;
        while i < m_cycles {
            i += self.cpu.run_instruction(&mut self.mmu) as u64;
        }

    }

}

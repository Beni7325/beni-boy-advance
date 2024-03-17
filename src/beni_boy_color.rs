use crate::cpu::Cpu;
use crate::mmu::Mmu;


pub struct BeniBoyColor {
    cpu: Cpu,
    pub mmu: Mmu
}

impl BeniBoyColor {

    pub fn new(rom_path: &str) -> BeniBoyColor {
        BeniBoyColor { cpu: Cpu::new(), mmu: Mmu::new(rom_path) }
    }

    pub fn tick(&mut self) {
        let instr_cycles = self.cpu.run_instruction(&mut self.mmu);
        self.mmu.tick_components(instr_cycles as u64);
    }

}

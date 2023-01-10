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

}

use crate::{cartridge::Cartridge, ppu::Ppu};

pub struct Mmu {
    cart: Cartridge,
    ppu: Ppu,

    wram: [u8; 0x2000],
    hram: [u8; 0x007F],
    io_regs: [u8; 0x0080]  // Temporary
}

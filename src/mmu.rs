use std::path;
use crate::{cartridge::Cartridge, ppu::Ppu};

pub struct Mmu {
    cart: Cartridge,
    ppu: Ppu,

    wram: Box<[u8; 0x2000]>,
    hram: Box<[u8; 0x007F]>,
    io_regs: [u8; 0x0080]  // Temporary
}

impl Mmu {

    pub fn new(rom_path: &str) -> Mmu {

        let cartridge = Cartridge::new(&rom_path);
        let cartridge = match cartridge {
            Ok(cart) => cart,
            Err(err) => {
                panic!();  // We'll deal with the error later...
            }
        };

        Mmu {
            cart: cartridge,
            ppu: Ppu::new(),
            wram: vec![0; 0x2000].into_boxed_slice().try_into().expect("Array size mismatch!"),
            hram: vec![0; 0x007F].into_boxed_slice().try_into().expect("Array size mismatch!"),
            io_regs: [0; 0x0080]
        }
    }

}

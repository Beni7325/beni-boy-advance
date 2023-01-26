use crate::{cartridge::Cartridge, ppu::Ppu};

pub struct Mmu {
    cart: Cartridge,
    ppu: Ppu,

    wram: Box<[u8; 0x2000]>,
    hram: Box<[u8; 0x007F]>,
    io_regs: [u8; 0x0080],  // Temporary
    pub interrupt_enable: u8,
    pub interrupt_flag: u8
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
            io_regs: [0; 0x0080],
            interrupt_enable: 0x00,
            interrupt_flag: 0xE1
        }
    }

}

impl Mmu {

    pub fn read_byte(&self, addr: u16) -> u8 {

        match addr {
            // ROM
            0x0000 ..= 0x7FFF => self.cart.read_rom(addr),

            // VRAM
            0x8000 ..= 0x9FFF => self.ppu.read_vram(addr - 0x8000),

            // External RAM
            0xA000 ..= 0xBFFF => self.cart.read_ram(addr - 0xA000),

            // WRAM 0
            0xC000 ..= 0xCFFF => self.wram[(addr - 0xC000) as usize],

            // WRAM 1-n
            0xD000 ..= 0xDFFF => self.wram[(addr - 0xC000) as usize],  // TODO: When converting to GBC, implement wram banking

            // ECHO
            0xE000 ..= 0xFDFF => self.wram[(addr - 0xE000) as usize],

            // OAM
            0xFE00 ..= 0xFE9F => self.ppu.read_oam(addr - 0xFE00),

            // Not Used
            0xFEA0 ..= 0xFEFF => 0xFF,

            // IO Regs
            0xFF00 ..= 0xFF7F => {
                match ((addr - 0xFF00) & 0x7F) as u8 {

                    // IF
                    0x0F => self.interrupt_flag,

                    // LY
                    0x44 => 0x90,

                    _ => self.io_regs[(addr - 0xFF00) as usize]
                }
            },

            // HRAM
            0xFF80 ..= 0xFFFE => self.hram[(addr - 0xFF80) as usize],

            // Interrupt Enable Reg
            0xFFFF            => self.interrupt_enable
        }
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        (self.read_byte(addr + 1) as u16) << 8 | self.read_byte(addr) as u16
    }

    pub fn write_byte(&mut self, addr: u16, data: u8) {
        
        match addr {
            // ROM
            0x0000 ..= 0x7FFF => self.cart.write_rom(addr, data),

            // VRAM
            0x8000 ..= 0x9FFF => self.ppu.write_vram(addr - 0x8000, data),

            // External RAM
            0xA000 ..= 0xBFFF => self.cart.write_ram(addr - 0xA000, data),

            // WRAM 0
            0xC000 ..= 0xCFFF => self.wram[(addr - 0xC000) as usize] = data,

            // WRAM 1-n
            0xD000 ..= 0xDFFF => self.wram[(addr - 0xC000) as usize] = data,  // TODO: When converting to GBC, implement wram banking

            // ECHO
            0xE000 ..= 0xFDFF => self.wram[(addr - 0xE000) as usize] = data,  // TODO: Read more on bug with echo and OAM DMA

            // OAM
            0xFE00 ..= 0xFE9F => self.ppu.write_oam(addr - 0xFE00, data),

            // Not Used
            0xFEA0 ..= 0xFEFF => { /* Can't write here */ },

            // IO Regs
            0xFF00 ..= 0xFF7F => {
                self.io_regs[(addr - 0xFF00) as usize] = data;
                if addr == 0xFF && data == 0x81 {
                    print!("{}", self.io_regs[1] as char);
                }
                match ((addr - 0xFF00) & 0x7F) as u8 {

                    //
                    0x02 => {
                        if data == 0x81 {
                            print!("{}", self.io_regs[1] as char)
                        }
                        self.io_regs[(addr - 0xFF00) as usize] = data
                    },

                    // IF
                    0x0F => self.interrupt_flag = data,

                    _ => self.io_regs[(addr - 0xFF00) as usize] = data
                }
            }

            // HRAM
            0xFF80 ..= 0xFFFE => self.hram[(addr - 0xFF80) as usize] = data,

            // Interrupt Enable Reg
            0xFFFF            => self.interrupt_enable = data
        }

    }

    pub fn write_word(&mut self, addr: u16, data: u16) {
        self.write_byte(addr, (data & 0x00FF) as u8);
        self.write_byte( addr+1, ((data & 0xFF00) >> 8) as u8);
    }

}

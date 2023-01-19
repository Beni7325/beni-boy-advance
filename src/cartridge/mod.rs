use std::path::Path;


pub struct Cartridge {
    rom: Box<[u8]>,
    rom_size: usize,
    rom_bank: usize,

    external_ram: Box<[u8]>,
    external_ram_bank: usize
}

#[derive(Debug)]
pub enum CartridgeError {
    RomReadError,
    InvalidRomError
}

impl Cartridge {

    pub fn new<P: AsRef<Path>>(rom_path: &P)-> Result<Cartridge, CartridgeError> {

        let rom_contents = std::fs::read(rom_path);
        let rom = match rom_contents {
            Ok(rom) => rom,
            Err(_) => return Err(CartridgeError::RomReadError)
        };
        
        // First we need to check if the ROM file is long enough to check some fields
        // from its header
        let rom_size = rom.len();
        if rom_size < 0x150 {
            return Err(CartridgeError::InvalidRomError)
        }

        match get_rom_size(rom[0x148]) {
            Some(size) if size == rom_size => {},
            _ => return Err(CartridgeError::InvalidRomError)
        }

        let external_ram_size = match get_ext_ram_size(rom[0x149]) {
            Some(size) => size,
            None => return Err(CartridgeError::InvalidRomError)
        };

        Ok(Cartridge { 
            rom: rom.into_boxed_slice(),
            rom_size: rom_size,
            rom_bank: 0x4000,
            external_ram: vec![0; external_ram_size].into_boxed_slice(),
            external_ram_bank: 0x0000
        })
        
    }

    pub fn read_rom(&self, addr: u16) -> u8 {
        self.rom[addr as usize]
    }

    pub fn write_rom(&mut self, addr: u16, data: u8) {

    }

    pub fn read_ram(&self, addr: u16) -> u8 {
        self.external_ram[addr as usize]   // TODO: check there is ram
    }

    pub fn write_ram(&mut self, addr: u16, data: u8) {
        self.external_ram[addr as usize] = data;   // TODO: check there is ram
    }

}

fn get_ext_ram_size(byte: u8) -> Option<usize> {
    match byte {
        0x00 => Some(0x00000),  // No RAM
        0x02 => Some(0x02000),  // 1 Bank (8 KiB)
        0x03 => Some(0x08000),  // 4 Banks (32 KiB)
        0x04 => Some(0x20000),  // 16 Banks (128 KiB)
        0x05 => Some(0x10000),  // 8 Banks (64 KiB)
        _ => None
    }
}

fn get_rom_size(byte: u8) -> Option<usize> {
    match byte {
        0x00 => Some(0x008000),  // 32 KiB
        0x01 => Some(0x010000),  // 64 KiB
        0x02 => Some(0x020000),  // 128 KiB
        0x03 => Some(0x040000),  // 256 KiB
        0x04 => Some(0x080000),  // 512 KiB
        0x05 => Some(0x100000),  // 1 MiB
        0x06 => Some(0x200000),  // 2 MiB
        0x07 => Some(0x400000),  // 4 MiB
        0x08 => Some(0x800000),  // 8 MiB
        _ => None
    }
}

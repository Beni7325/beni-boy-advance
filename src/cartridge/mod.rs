use std::path::Path;


const EXTERNAL_RAM_SIZES: [usize; 4] = [0, 2048, 8192, 32768];

pub struct Cartridge {
    rom: Box<[u8]>,
    rom_size: usize,
    rom_bank: usize,

    external_ram: Box<[u8]>,
    external_ram_bank: usize
}

pub enum CartridgeError {
    RomReadError,
    RomSizeError
}

impl Cartridge {

    pub fn new<P: AsRef<Path>>(rom_path: &P)-> Result<Cartridge, CartridgeError> {

        let rom_contents = std::fs::read(rom_path);
        let rom = match rom_contents {
            Ok(rom) => rom,
            Err(_) => return Err(CartridgeError::RomReadError)
        };
        
        let rom_size = rom.len();
        if rom_size < 0x150 {
            return Err(CartridgeError::RomSizeError)
        }

        let external_ram_size = EXTERNAL_RAM_SIZES[rom[0x149] as usize];

        Ok(Cartridge { 
            rom: rom.into_boxed_slice(),
            rom_size: rom_size,
            rom_bank: 0x4000,
            external_ram: vec![0; external_ram_size].into_boxed_slice(),
            external_ram_bank: 0x0000
        })
        
    }

}

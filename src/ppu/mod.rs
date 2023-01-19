pub struct Ppu {
    vram: Box<[u8; 0x2000]>,
    oam: Box<[u8; 0x00A0]>
}

impl Ppu {

    pub fn new() -> Ppu {
        Ppu {
            vram: vec![0; 0x2000].into_boxed_slice().try_into().expect("Array size mismatch!"),
            oam: vec![0; 0x00A0].into_boxed_slice().try_into().expect("Array size mismatch!")
        }
    }

    pub fn read_vram(&self, addr: u16) -> u8 {
        self.vram[addr as usize]
    }

    pub fn write_vram(&mut self, addr: u16, data: u8) {
        self.vram[addr as usize] = data;
    }

    pub fn read_oam(&self, addr: u16) -> u8 {
        self.oam[addr as usize]
    }

    pub fn write_oam(&mut self, addr: u16, data: u8) {
        self.oam[addr as usize] = data;
    }

}

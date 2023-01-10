pub struct Ppu {
    vram: [u8; 0x2000],
    oam: [u8; 0x00A0]
}

impl Ppu {

    pub fn new() -> Ppu {
        Ppu {
            vram: [0; 0x2000],
            oam: [0; 0x00A0]
        }
    }

}

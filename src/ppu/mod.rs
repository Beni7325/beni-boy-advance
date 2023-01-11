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

}

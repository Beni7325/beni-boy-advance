pub struct Cartridge {
    rom: Box<[u8]>,
    external_ram: Box<[u8]>
}
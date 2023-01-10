mod cpu;
mod mmu;
mod cartridge;
mod ppu;
mod beni_boy_color;


fn main() {
    let emu = beni_boy_color::BeniBoyColor::new("roms/tests/cpu_instrs/cpu_instrs.gb");
}

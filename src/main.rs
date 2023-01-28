use std::env;

mod cpu;
mod mmu;
mod cartridge;
mod ppu;
mod timer;
mod beni_boy_color;


fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!()
    }

    let rom_path = &args[1];
    let mut emu = beni_boy_color::BeniBoyColor::new(rom_path);
    emu.run(2_000_000/4);
}

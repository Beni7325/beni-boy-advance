extern crate sdl2;

use bytemuck::cast_slice;
use sdl2::{event::{Event, WindowEvent}, pixels::PixelFormatEnum};
use std::{env, time::{Duration, Instant}};

mod cpu;
mod mmu;
mod cartridge;
mod ppu;
mod timer;
mod beni_boy_color;

const FRAME_RATE: f64 = 59.7275;
const M_CYCLES_PER_FRAME: u32 = 70224;
const SCREEN_SIZE_MULTIPLIER: u32 = 6;

fn main() {

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!()
    }

    let rom_path = &args[1];
    let mut gbc = beni_boy_color::BeniBoyColor::new(rom_path);

    // Setup SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("BeniBoy Color", ppu::SCREEN_WIDTH as u32 * SCREEN_SIZE_MULTIPLIER, ppu::SCREEN_HEIGHT as u32 * SCREEN_SIZE_MULTIPLIER)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .build()
        .expect("failed to build window's canvas");
    
    let texture_creator = canvas.texture_creator();
    
    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::ARGB8888, ppu::SCREEN_WIDTH as u32, ppu::SCREEN_HEIGHT as u32).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    // Emu/Render loop
    'main_loop: loop {

        let start = Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::Window { win_event: WindowEvent::Close, .. } => {
                    break 'main_loop
                },
                _ => {}
            }
        }

        for _ in 0..M_CYCLES_PER_FRAME {
            gbc.tick();
        }

        //canvas.clear();
        let _ = texture.update(None, cast_slice(gbc.mmu.ppu.screen.as_ref()), ppu::SCREEN_WIDTH * 4);
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();

        let frame_duration = Duration::from_secs_f64(1.0 / FRAME_RATE);
        std::thread::sleep(
            frame_duration
                .checked_sub(start.elapsed())
                .unwrap_or(Duration::ZERO)
        );
    }

}

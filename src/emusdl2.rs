use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

use crate::chip8::Chip8;

/// An emulator of the Chip8 model that uses SDL2
pub struct EmuSdl2 {
    chip8: Chip8,
    fps: u16,
    mul: u16,
}

impl EmuSdl2 {
    pub fn new(program: Vec<u8>, fps: u16, mul: u16) -> Self {
        EmuSdl2 {
            chip8: Chip8::new(program),
            fps,
            mul,
        }
    }

    pub fn run(&mut self) {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video
            .window("Chip8 Emulator", 800, 400)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let mut events = sdl.event_pump().unwrap();

        'main: loop {
            let t = Instant::now();
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();

            // Handle input
            for event in events.poll_iter() {
                match event {
                    // Quit
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'main,
                    _ => {}
                }
            }

            for _ in 0..self.mul {
                self.chip8.step();
            }

            // Decrement delay timer if non-zero
            if self.chip8.dt > 0 {
                self.chip8.dt -= 1;
            }

            // Decrement sound timer if non-zero and play sound
            if self.chip8.st > 0 {
                self.chip8.st -= 1;
            }

            // Draw display if display_updated
            if self.chip8.display_updated {
                canvas.present();
                self.chip8.display_updated = false;
            }

            let sleep_duration =
                (1000_000_000 as i64 / self.fps as i64) - t.elapsed().as_nanos() as i64;

            println!("Sleeping {} ns", sleep_duration);
            if sleep_duration >= 0 {
                sleep(Duration::new(0, sleep_duration as u32));
            }
        }
    }
}

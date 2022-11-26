use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point};

use crate::chip8::{Chip8, DISPLAY_HEIGHT, DISPLAY_WIDTH};

/// An emulator of the Chip8 model using SDL2 for keyboard input, video and sound
pub struct EmuSdl2 {
    /// The Chip8 instance to run
    chip8: Chip8,
    /// Frames per second. 60 is the default
    fps: u16,
    /// Target instructions per second as multiplier of fps. 10 is the recommended default
    mul: u16,
    /// Scale display by this number. Original display is 64x32 pixels. 10 or more is the recommended default
    scale: u8,
}

impl EmuSdl2 {
    /// Create a new instance passing in binary program code and options
    pub fn new(program: Vec<u8>, fps: u16, mul: u16, scale: u8) -> Self {
        EmuSdl2 {
            chip8: Chip8::new(program),
            fps,
            mul,
            scale,
        }
    }

    /// Run the Chip8 at desired fps and instruction multiplier rate. Use SDL2 to obtain input and render graphics as well as sound
    pub fn run(&mut self) {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let window = video
            .window(
                "Chip8 Emulator",
                DISPLAY_WIDTH as u32 * self.scale as u32,
                DISPLAY_HEIGHT as u32 * self.scale as u32,
            )
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window
            .into_canvas()
            .present_vsync()
            .accelerated()
            .build()
            .unwrap();
        // The logical size is set to the size of the Chip8 display. It makes it possible to draw single pixels at the correct position and get a scaled display automatically
        canvas
            .set_logical_size(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
            .unwrap();

        println!(
            "{:?}, default_pixel_format: {:?}, scale: {:?}, logical_size: {:?}, output_size: {:?}",
            canvas.info(),
            canvas.default_pixel_format(),
            canvas.scale(),
            canvas.logical_size(),
            canvas.output_size().unwrap()
        );

        let mut events = sdl.event_pump().unwrap();

        'main: loop {
            let t = Instant::now();

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

            // Step the Chip8 mul times
            for _ in 0..self.mul {
                self.chip8.step();
            }

            // Decrement delay timer if non-zero
            if self.chip8.dt > 0 {
                self.chip8.dt -= 1;
            }

            // Decrement sound timer if non-zero and play sound
            if self.chip8.st > 0 {
                // TODO Play sound
                self.chip8.st -= 1;
            }

            // Draw display if Chip8 indicates display is updated
            if self.chip8.display_update {
                // TODO Make background color an option
                canvas.set_draw_color(Color::RGB(32, 32, 32));
                canvas.clear();
                // TODO Make foreground color an option
                canvas.set_draw_color(Color::RGB(102, 255, 102));

                for y in 0..DISPLAY_HEIGHT {
                    for x in 0..DISPLAY_WIDTH {
                        if self.chip8.display[y][x] {
                            canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
                        }
                    }
                }
                canvas.present();
                println!("Display updated");
                self.chip8.display_update = false; // Chip8 will set this to true whenever something changes on screen
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

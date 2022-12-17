use crate::chip8::{Chip8, DISPLAY_HEIGHT, DISPLAY_WIDTH};
use sdl2::{
    audio::{AudioCallback, AudioSpecDesired, AudioStatus},
    render::BlendMode,
};
use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
    pixels::Color,
    rect::Point,
};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

/// An emulator of the Chip8 model using SDL2 for keyboard input, video and sound
pub struct EmuSdl2 {
    /// The Chip8 instance to run
    chip8: Chip8,
    /// Frames per second. 60 is the default
    fps: u16,
    /// Target instructions per second as multiplier of fps. 20 is the recommended default
    mul: u16,
    /// Scale display by this number. Original display is 64x32 pixels. 10 or more is the recommended default
    scale: u8,
    /// Foreground color
    color: u32,
    /// Background color
    background: u32,
    /// Pitch of buzzer
    pitch: u16,
}

#[derive(Debug)]
pub struct Options {
    pub fps: u16,
    pub mul: u16,
    pub scale: u8,
    pub color: u32,
    pub background: u32,
    pub pitch: u16,
}

impl EmuSdl2 {
    /// Create a new instance passing in binary program code and options
    pub fn new(chip8: Chip8, options: Options) -> Self {
        EmuSdl2 {
            chip8,
            fps: options.fps,
            mul: options.mul,
            scale: options.scale,
            color: options.color,
            background: options.background,
            pitch: options.pitch,
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

        // Support alpha blending
        canvas.set_blend_mode(BlendMode::Blend);

        let background_color = Color::RGBA(
            ((self.background & 0xff0000) >> 16) as u8,
            ((self.background & 0x00ff00) >> 8) as u8,
            (self.background & 0x0000ff) as u8,
            ((self.background & 0xff000000) >> 24) as u8,
        );

        let foreground_color = Color::RGBA(
            ((self.color & 0xff0000) >> 16) as u8,
            ((self.color & 0x00ff00) >> 8) as u8,
            (self.color & 0x0000ff) as u8,
            ((self.color & 0xff000000) >> 24) as u8,
        );

        // Create a grid as a texture
        let texture_creator = canvas.texture_creator();
        let mut grid = texture_creator
            .create_texture_target(
                texture_creator.default_pixel_format(),
                (DISPLAY_WIDTH * self.scale as usize) as u32,
                (DISPLAY_HEIGHT * self.scale as usize) as u32,
            )
            .unwrap();
        grid.set_blend_mode(BlendMode::Blend);

        canvas
            .with_texture_canvas(&mut grid, |c| {
                let mut grid_color = background_color;
                grid_color.a = 0x22;
                c.set_draw_color(grid_color);
                // Draw horizontal lines
                for y in 0..(DISPLAY_HEIGHT * self.scale as usize) {
                    if y % (self.scale as usize) == 0 {
                        c.draw_line(
                            (0, y as i32),
                            ((self.scale as usize * DISPLAY_WIDTH) as i32, y as i32),
                        )
                        .unwrap();
                    }
                }
                // Draw vertical lines
                for x in 0..(DISPLAY_WIDTH * self.scale as usize) {
                    if x % (self.scale as usize) == 0 {
                        c.draw_line(
                            (x as i32, 0),
                            (x as i32, (self.scale as usize * DISPLAY_HEIGHT) as i32),
                        )
                        .unwrap();
                    }
                }
            })
            .unwrap();

        // The logical size is set to the size of the Chip8 display. It makes it possible to draw single pixels at the correct position and get a scaled display automatically
        canvas
            .set_logical_size(DISPLAY_WIDTH as u32, DISPLAY_HEIGHT as u32)
            .unwrap();

        println!(
            "{:?}, default_pixel_format: {:?}, scale: {:?}, logical_size: {:?}, output_size: {:?}, render_target_supported: {:?}",
            canvas.info(),
            canvas.default_pixel_format(),
            canvas.scale(),
            canvas.logical_size(),
            canvas.output_size().unwrap(),
            canvas.render_target_supported()
        );

        // Audio
        let audio_subsystem = sdl.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        let device = audio_subsystem
            .open_playback(None, &desired_spec, |spec| {
                // initialize the audio callback
                SquareWave {
                    phase_inc: self.pitch as f32 / spec.freq as f32,
                    phase: 0.0,
                    volume: 0.25,
                }
            })
            .unwrap();

        println!(
            "{} {:?}",
            audio_subsystem.current_audio_driver(),
            audio_subsystem
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
                    Event::KeyDown {
                        scancode: Some(scancode),
                        ..
                    } => {
                        if let Some(keycode) = self.keymap(scancode) {
                            self.chip8.keyboard[keycode] = true;
                            #[cfg(debug_assertions)]
                            eprintln!("Key {:0x} down", keycode);
                        }
                    }
                    Event::KeyUp {
                        scancode: Some(scancode),
                        ..
                    } => {
                        if let Some(keycode) = self.keymap(scancode) {
                            self.chip8.keyboard[keycode] = false;
                            #[cfg(debug_assertions)]
                            eprintln!("Key {:0x} up", keycode);
                        }
                    }
                    _ => {}
                }
            }

            // Step the Chip8 mul times
            for _ in 0..self.mul {
                self.chip8.step();

                if self.chip8.quirks.display_wait && self.chip8.display_update {
                    break;
                }
            }

            // Decrement delay timer if non-zero
            if self.chip8.dt > 0 {
                self.chip8.dt -= 1;
            }

            // Decrement sound timer if non-zero and play sound
            if self.chip8.st > 0 {
                if device.status() != AudioStatus::Playing {
                    device.resume();
                }
                self.chip8.st -= 1;
            } else if device.status() != AudioStatus::Paused {
                device.pause();
            }

            // Draw display if Chip8 indicates display is updated
            if self.chip8.display_update {
                canvas.set_draw_color(background_color);
                canvas.clear();
                canvas.set_draw_color(foreground_color);

                for y in 0..DISPLAY_HEIGHT {
                    for x in 0..DISPLAY_WIDTH {
                        if self.chip8.display[y][x] {
                            canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
                        }
                    }
                }

                // Copy grid texture on top (could be configurable)
                canvas.copy(&grid, None, None).unwrap();

                canvas.present();

                #[cfg(debug_assertions)]
                eprintln!("Display updated");

                self.chip8.display_update = false; // Chip8 will set this to true whenever something changes on screen
            }

            let sleep_duration =
                (1_000_000_000_i64 / self.fps as i64) - t.elapsed().as_nanos() as i64;

            #[cfg(debug_assertions)]
            eprintln!("Sleeping {} ns", sleep_duration);

            if sleep_duration >= 0 {
                sleep(Duration::new(0, sleep_duration as u32));
            }
        }
    }

    fn keymap(&self, scancode: Scancode) -> Option<usize> {
        match scancode {
            Scancode::Num1 => Some(1),
            Scancode::Num2 => Some(2),
            Scancode::Num3 => Some(3),
            Scancode::Num4 => Some(0xC),
            Scancode::Q => Some(4),
            Scancode::W => Some(5),
            Scancode::E => Some(6),
            Scancode::R => Some(0xD),
            Scancode::A => Some(7),
            Scancode::S => Some(8),
            Scancode::D => Some(9),
            Scancode::F => Some(0xE),
            Scancode::Z => Some(0xA),
            Scancode::X => Some(0),
            Scancode::C => Some(0xB),
            Scancode::V => Some(0xF),
            _ => None,
        }
    }
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

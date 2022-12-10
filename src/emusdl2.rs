use std::{
    thread::sleep,
    time::{Duration, Instant},
};

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

impl EmuSdl2 {
    /// Create a new instance passing in binary program code and options
    pub fn new(
        program: Vec<u8>,
        fps: u16,
        mul: u16,
        scale: u8,
        color: u32,
        background: u32,
        pitch: u16,
    ) -> Self {
        EmuSdl2 {
            chip8: Chip8::new(program),
            fps,
            mul,
            scale,
            color,
            background,
            pitch,
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

        // Support alpha blending
        canvas.set_blend_mode(BlendMode::Blend);

        println!(
            "{:?}, default_pixel_format: {:?}, scale: {:?}, logical_size: {:?}, output_size: {:?}",
            canvas.info(),
            canvas.default_pixel_format(),
            canvas.scale(),
            canvas.logical_size(),
            canvas.output_size().unwrap()
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

                if self.chip8.display_update {
                    break; // QUIRK
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
                canvas.set_draw_color(Color::RGBA(
                    ((self.background & 0xff0000) >> 16) as u8,
                    ((self.background & 0x00ff00) >> 8) as u8,
                    (self.background & 0x0000ff) as u8,
                    ((self.background & 0xff000000) >> 24) as u8,
                ));
                canvas.clear();
                canvas.set_draw_color(Color::RGBA(
                    ((self.color & 0xff0000) >> 16) as u8,
                    ((self.color & 0x00ff00) >> 8) as u8,
                    (self.color & 0x0000ff) as u8,
                    ((self.color & 0xff000000) >> 24) as u8,
                ));

                for y in 0..DISPLAY_HEIGHT {
                    for x in 0..DISPLAY_WIDTH {
                        if self.chip8.display[y][x] {
                            canvas.draw_point(Point::new(x as i32, y as i32)).unwrap();
                        }
                    }
                }
                canvas.present();

                #[cfg(debug_assertions)]
                eprintln!("Display updated");

                self.chip8.display_update = false; // Chip8 will set this to true whenever something changes on screen
                continue; // We continue without waiting here since Vsync is on and there will be a re-draw of the screen
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

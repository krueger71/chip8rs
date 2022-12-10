mod chip8;
mod emusdl2;

use std::path::PathBuf;

use clap::Parser;
use clap_num::maybe_hex;
use emusdl2::EmuSdl2;

use crate::{
    chip8::{Chip8, Quirks},
    emusdl2::Options,
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// A simple Chip8 emulator that uses SDL
struct Cli {
    /// Path to the binary Chip8 program to run
    program: PathBuf,
    /// Frames per second
    #[arg(short, long, default_value_t = 60)]
    fps: u16,
    /// Instruction multiplier (instructions per frame)
    #[arg(short, long, default_value_t = 20)]
    mul: u16,
    /// Scale of display
    #[arg(short, long, default_value_t = 10)]
    scale: u8,
    /// Foreground color. Format ARGB8888 (hex possible, e.g. 0xff0000ff)
    #[arg(short, long, value_parser=maybe_hex::<u32>, default_value_t = 0xff33ff00)]
    color: u32,
    /// Background color. Format ARGB8888 (hex possible, e.g. 0xff111111)
    #[arg(short, long, value_parser=maybe_hex::<u32>, default_value_t = 0xff111111)]
    background: u32,
    /// Pitch of buzzer in Hz
    #[arg(short, long, default_value_t = 220)]
    pitch: u16,
    /// Quirk: AND, OR, XOR reset VF to zero
    #[arg(long)]
    quirk_vf_reset: bool,
    /// Quirk: Memory load/store registers operations increment I
    #[arg(long)]
    quirk_memory: bool,
    /// Quirk: Only one draw operation per frame
    #[arg(long)]
    quirk_draw: bool,
    /// Quirk: Drawing operations clip instead of wrap
    #[arg(long)]
    quirk_clipping: bool,
    /// Quirk: Shifting operations use VY instead of only VX
    #[arg(long)]
    quirk_shifting: bool,
    /// Quirk: Jump with offset operation BNNN will work as BXNN.
    #[arg(long)]
    quirk_jumping: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);

    let program = std::fs::read(&cli.program).expect("could not read file");

    let quirks = Quirks {
        quirk_vf_reset: cli.quirk_vf_reset,
        quirk_memory: cli.quirk_memory,
        quirk_draw: cli.quirk_draw,
        quirk_clipping: cli.quirk_clipping,
        quirk_shifting: cli.quirk_shifting,
        quirk_jumping: cli.quirk_jumping,
    };

    let chip8 = Chip8::new(program, quirks);

    let options: Options = Options {
        fps: cli.fps,
        mul: cli.mul,
        scale: cli.scale,
        color: cli.color,
        background: cli.background,
        pitch: cli.pitch,
    };

    let mut emusdl = EmuSdl2::new(chip8, options);

    emusdl.run();
}

mod chip8;
mod emusdl2;

use std::path::PathBuf;

use clap::Parser;
use clap_num::maybe_hex;
use emusdl2::EmuSdl2;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
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
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);

    let program = std::fs::read(&cli.program).expect("could not read file");
    let mut emusdl = EmuSdl2::new(
        program,
        cli.fps,
        cli.mul,
        cli.scale,
        cli.color,
        cli.background,
        cli.pitch,
    );
    emusdl.run();
}

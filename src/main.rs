mod chip8;
mod emusdl2;

use std::path::PathBuf;

use clap::Parser;
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
}

fn main() {
    let cli = Cli::parse();

    println!("{:?}", cli);

    let program = std::fs::read(&cli.program).expect("could not read file");
    let mut emusdl = EmuSdl2::new(program, cli.fps, cli.mul, cli.scale);
    emusdl.run();
}

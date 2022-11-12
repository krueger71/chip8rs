//! A Chip8 emulator based on information and inspiration from:
//!
//! * [https://en.wikipedia.org/wiki/CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
//! * [https://tonisagrista.com/blog/2021/chip8-spec/](https://tonisagrista.com/blog/2021/chip8-spec/)
//! * [http://devernay.free.fr/hacks/chip8/C8TECH10.HTM](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
//! * [https://chip-8.github.io/](https://chip-8.github.io/)
//!
//! The purpose of the implementation is to learn Rust and lower level programming in general.

/// Memory size in bytes
pub const MEMORY_SIZE:usize = 4096;
/// Number of general purpose registers
pub const NUMBER_OF_REGISTERS:usize = 16;

/// The virtual machine, Chip8
pub struct Chip8 {
    /// RAM
    pub memory: [u8; MEMORY_SIZE],
    /// General purpose registers
    pub registers: [u8; NUMBER_OF_REGISTERS],
    /// Delay timer register
    pub dt: u8,
    /// Sound timer register
    pub st: u8,
    /// Index regiser
    pub i: u16,
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u8
}



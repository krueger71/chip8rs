//! A Chip8 emulator based on information and inspiration from:
//!
//! * [https://en.wikipedia.org/wiki/CHIP-8](https://en.wikipedia.org/wiki/CHIP-8)
//! * [https://tonisagrista.com/blog/2021/chip8-spec/](https://tonisagrista.com/blog/2021/chip8-spec/)
//! * [http://devernay.free.fr/hacks/chip8/C8TECH10.HTM](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
//! * [https://chip-8.github.io/](https://chip-8.github.io/)
//! * [https://tobiasvl.github.io/blog/write-a-chip-8-emulator/](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/)
//! * [https://github.com/JohnEarnest/Octo](https://github.com/JohnEarnest/Octo)
//! * [https://github.com/mattmikolay/chip-8](https://github.com/mattmikolay/chip-8)
//!
//! The purpose of the implementation is both to learn Rust and basic emulator programming.

/// Memory size in bytes
pub const MEMORY_SIZE: usize = 4096;
/// Program start
pub const PROGRAM_START: usize = 0x200;
/// Number of general purpose registers
pub const NUMBER_OF_REGISTERS: usize = 16;
/// Size of stack
pub const STACK_SIZE: usize = 16;
/// Width of display in pixels
pub const DISPLAY_WIDTH: usize = 64;
/// Height of display in pixels
pub const DISPLAY_HEIGHT: usize = 32;
/// Size of display in bytes
pub const DISPLAY_SIZE: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT) / 8;
/// Size of fonts in bytes
pub const FONTS_SIZE: usize = 16 * 5;
/// Default fonts
const FONTS: [u8; FONTS_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

/// The virtual machine for Chip8
pub struct Chip8 {
    /// RAM
    pub memory: [u8; MEMORY_SIZE],
    /// General purpose registers
    pub registers: [u8; NUMBER_OF_REGISTERS],
    /// Delay timer register
    pub dt: u8,
    /// Sound timer register
    pub st: u8,
    /// Index register
    pub i: u16,
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u8,
    /// Stack
    pub stack: [u16; STACK_SIZE],
    /// Display "buffer"
    pub display: [u8; DISPLAY_SIZE],
}

impl Chip8 {
    pub fn new(program: Vec<u8>) -> Self {
        let mut memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];

        memory[..FONTS_SIZE].copy_from_slice(&FONTS); // Load fonts from address 0x0000
        memory[PROGRAM_START..(PROGRAM_START + program.len())].copy_from_slice(&program); // Load program at PROGRAM_START

        Chip8 {
            memory,
            registers: [0; NUMBER_OF_REGISTERS],
            dt: 0,
            st: 0,
            i: 0,
            pc: PROGRAM_START as u16,
            sp: 0,
            stack: [0; STACK_SIZE],
            display: [0; DISPLAY_SIZE],
        }
    }

    pub fn step(&mut self) {
        let instr = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[1 + self.pc as usize] as u16);
        self.pc += 2;
        let opcode = instr & 0xF000 >> 8;

        match opcode {
            _ => println!("op = {:04x}", opcode),
        }
    }
}

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
const MEMORY_SIZE: usize = 4096;
/// Program start
const PROGRAM_START: usize = 0x200;
/// Number of general purpose registers
const NUMBER_OF_REGISTERS: usize = 16;
/// Size of stack
const STACK_SIZE: usize = 16;
/// Width of display in pixels
pub const DISPLAY_WIDTH: usize = 64;
/// Height of display in pixels
pub const DISPLAY_HEIGHT: usize = 32;
/// Size of display in bytes
const DISPLAY_SIZE: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT) / 8;
/// Size of fonts in bytes
const FONTS_SIZE: usize = 16 * 5;
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
#[derive(Debug)]
#[allow(dead_code)]
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
    /// Display has been updated. Redraw the display on target and set to false
    pub display_updated: bool,
    /// Sound should play
    pub play_sound: bool,
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
            display_updated: true,
            play_sound: false,
        }
    }

    pub fn step(&mut self) {
        let instr = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[1 + self.pc as usize] as u16);
        self.pc += 2;

        // 0xIXYN,0x__NN,0x_NNN
        let i = ((instr & 0xF000) >> 12) as u8;
        let x = ((instr & 0x0F00) >> 8) as u8;
        let y = ((instr & 0x00F0) >> 4) as u8;
        let n = (instr & 0x000F) as u8;
        let nn = (instr & 0x00FF) as u8;
        let nnn = instr & 0x0FFF;

        println!(
            "instr = {:04x}, i = {:x}, x = {:x}, y = {:x}, n = {:x}, nn = {:02x}, nnn = {:03x}, pc = {:04x}",
            instr, i, x, y, n, nn, nnn, self.pc
        );

        /*

        Start with these
        00E0 (clear screen)
        1NNN (jump)
        6XNN (set register VX)
        7XNN (add value to register VX)
        ANNN (set index register I)
        DXYN (display/draw)

        */
        match i {
            0 => match nn {
                0xe0 => {
                    // 00E0 Clear the screen
                    self.display.fill(0);
                    self.display_updated = true;
                }
                _ => {
                    println!("instr = {:04x} not decoded!", instr);
                }
            },
            1 => {
                // 1NNN Jump to NNN
                self.pc = nnn;
            }
            6 => {
                // 6XNN set register VX to NN
                self.registers[x as usize] = nn;
            }
            7 => {
                // 7XNN add NN to register VX
                self.registers[x as usize] = (self.registers[x as usize] as u16 + nn as u16) as u8;
            }
            0xA => {
                // ANNN set index register I
                self.i = nnn;
            }
            0xD => {
                // DXYN draw
            }
            _ => {
                println!("instr = {:04x} not decoded!", instr);
            }
        }

        //println!("pc = {:04x}", self.pc);
    }
}

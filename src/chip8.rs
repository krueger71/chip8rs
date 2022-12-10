//! A Chip8 model
use crate::chip8::Instruction::*;

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
/// Size of the keyboard
pub const KEYBOARD_SIZE: usize = 16;

/// The virtual machine for Chip8
#[derive(Debug)]
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
    pub i: usize,
    /// Program counter
    pub pc: usize,
    /// Stack pointer
    pub sp: usize,
    /// Stack
    pub stack: [usize; STACK_SIZE],

    /// Display "buffer" output as 2-d array of bool
    pub display: [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
    /// Display has been updated. Redraw the display on target and set to false
    pub display_update: bool,
    /// Keyboard input as array of bool
    pub keyboard: [bool; KEYBOARD_SIZE],
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
            pc: PROGRAM_START,
            sp: 0,
            stack: [0; STACK_SIZE],
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            display_update: false,
            keyboard: [false; KEYBOARD_SIZE],
        }
    }

    /// Fetch, decode and execute one instruction
    pub fn step(&mut self) {
        let instr = self.fetch();
        let instr = Chip8::decode(instr);
        self.execute(instr);
    }

    /// Fetch one instruction from memory at current program counter
    fn fetch(&self) -> u16 {
        (self.memory[self.pc] as u16) << 8 | (self.memory[1 + self.pc] as u16)
    }

    /// Decode an instruction
    fn decode(instr: u16) -> Instruction {
        let i = ((instr & 0xF000) >> 12) as u8;
        let x = ((instr & 0x0F00) >> 8) as usize;
        let y = ((instr & 0x00F0) >> 4) as usize;
        let n = (instr & 0x000F) as u8;
        let nn = (instr & 0x00FF) as u8;
        let nnn = (instr & 0x0FFF) as usize;

        match i {
            0 => match nnn {
                0x0E0 => Cls,
                0x0EE => Ret,
                _ => Sys(nnn),
            },
            1 => Jmp(nnn),
            2 => Call(nnn),
            3 => Skeb(x, nn),
            4 => Skneb(x, nn),
            5 => Ske(x, y),
            6 => Ldb(x, nn),
            7 => Addb(x, nn),
            8 => match n {
                0 => Ld(x, y),
                1 => Or(x, y),
                2 => And(x, y),
                3 => Xor(x, y),
                4 => Add(x, y),
                5 => Sub(x, y),
                6 => Shr(x, y),
                7 => Subr(x, y),
                0xE => Shl(x, y),
                _ => Err(instr),
            },
            9 => Skne(x, y),
            0xA => Ldi(nnn),
            0xB => Jmpz(nnn),
            0xC => Rnd(x, nn),
            0xD => Draw(x, y, n),
            0xE => match nn {
                0x9E => Skp(x),
                0xA1 => Sknp(x),
                _ => Err(instr),
            },
            0xF => match nn {
                0x07 => Ldft(x),
                0x0A => Ldkp(x),
                0x15 => Ldtt(x),
                0x18 => Ldst(x),
                0x1E => Addi(x),
                0x29 => Font(x),
                0x33 => Bcd(x),
                0x55 => Sreg(x),
                0x65 => Lreg(x),
                _ => Err(instr),
            },
            _ => Err(instr),
        }
    }

    /// Execute one instruction
    fn execute(&mut self, instr: Instruction) {
        #[cfg(debug_assertions)]
        eprintln!(
            "pc: {:04X} instr: {:04X?} regs: {:02X?}",
            self.pc, instr, self.registers
        );
        // Increment program counter before as a default for most instructions
        self.pc += 2;

        match instr {
            Sys(_) => {}
            Cls => {
                self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
                self.display_update = true;
            }
            Call(nnn) => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            Ret => {
                self.sp -= 1;
                self.pc = self.stack[self.sp];
            }
            Jmp(nnn) => {
                self.pc = nnn;
            }
            Skeb(x, nn) => {
                if self.registers[x] == nn {
                    self.pc += 2;
                }
            }
            Skneb(x, nn) => {
                if self.registers[x] != nn {
                    self.pc += 2;
                }
            }
            Ske(x, y) => {
                if self.registers[x] == self.registers[y] {
                    self.pc += 2;
                }
            }
            Skne(x, y) => {
                if self.registers[x] != self.registers[y] {
                    self.pc += 2;
                }
            }
            Ldb(x, nn) => {
                self.registers[x] = nn;
            }
            Addb(x, nn) => {
                self.registers[x] = (self.registers[x] as u16 + nn as u16) as u8;
            }
            Ld(x, y) => {
                self.registers[x] = self.registers[y];
            }
            Or(x, y) => {
                self.registers[x] |= self.registers[y];
            }
            And(x, y) => {
                self.registers[x] &= self.registers[y];
            }
            Xor(x, y) => {
                self.registers[x] ^= self.registers[y];
            }
            Add(x, y) => {
                let (result, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                self.registers[x] = result;

                if overflow {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }
            }
            Sub(x, y) => {
                let (result, overflow) = self.registers[x].overflowing_sub(self.registers[y]);
                self.registers[x] = result;

                if overflow {
                    self.registers[0xF] = 0;
                } else {
                    self.registers[0xF] = 1;
                }
            }
            Subr(x, y) => {
                let (result, overflow) = self.registers[y].overflowing_sub(self.registers[x]);
                self.registers[x] = result;

                if overflow {
                    self.registers[0xF] = 0;
                } else {
                    self.registers[0xF] = 1;
                }
            }
            Shr(x, _y) => {
                let val = self.registers[x];
                self.registers[x] = val >> 1;
                self.registers[0xF] = val & 1;
            }
            Shl(x, _y) => {
                let val = self.registers[x];
                self.registers[x] = val << 1;
                self.registers[0xF] = 1 & (val >> 7);
            }
            Ldi(nnn) => {
                self.i = nnn;
            }
            Jmpz(nnn) => {
                self.pc = nnn + self.registers[0] as usize;
            }
            Rnd(x, nn) => {
                self.registers[x] = rand::random::<u8>() & nn;
            }
            Draw(x, y, n) => {
                let px = (self.registers[x] % (DISPLAY_WIDTH as u8)) as usize;
                let py = (self.registers[y] % (DISPLAY_HEIGHT as u8)) as usize;
                let idx = self.i as usize;
                let sprite = &self.memory[idx..(idx + n as usize)];
                self.registers[0xF] = 0;

                // Iterate over each individual bit in each byte of sprite
                // Set each bit according to the rules for DXYN draw in display
                // Sprites wrap-around immediately in this implementation, which is probably incorrect

                for (dy, byte) in sprite.iter().enumerate() {
                    if (py + dy) >= DISPLAY_HEIGHT {
                        // QUIRK
                        break;
                    }

                    for dx in 0..8 {
                        if (px + dx) >= DISPLAY_WIDTH {
                            // QUIRK
                            break;
                        }

                        let old = self.display[py + dy][px + dx];
                        let mut new = ((byte >> (7 - dx)) & 1) == 1;

                        if new {
                            if old {
                                new = false;
                                self.registers[0xF] = 1;
                            }

                            self.display[py + dy][px + dx] = new;
                            self.display_update = true;
                        }
                    }
                }

                #[cfg(debug_assertions)]
                eprintln!(
                    "DXYN px={} py={} n={} from i={:x} sprite={:02x?}",
                    px, py, n, self.i, sprite
                );
            }
            Skp(x) => {
                if self.keyboard[self.registers[x] as usize] {
                    self.pc += 2;
                }
            }
            Sknp(x) => {
                if !self.keyboard[self.registers[x] as usize] {
                    self.pc += 2;
                }
            }
            Ldft(x) => {
                self.registers[x] = self.dt;
            }
            Ldtt(x) => {
                self.dt = self.registers[x];
            }
            Ldst(x) => {
                self.st = self.registers[x];
            }
            Ldkp(x) => {
                let mut wait = true;
                for (key, pressed) in self.keyboard.iter().enumerate() {
                    if *pressed {
                        self.registers[x] = key as u8;
                        wait = false;
                        self.keyboard[key] = false;
                        break;
                    }
                }

                if wait {
                    self.pc -= 2;
                }
            }
            Addi(x) => {
                self.i += self.registers[x] as usize;
            }
            Font(x) => {
                self.i = (self.registers[x] * 5) as usize;
            }
            Bcd(x) => {
                let val = self.registers[x] as u16;
                self.memory[self.i] = (val % 1000 / 100) as u8;
                self.memory[self.i + 1] = (val % 100 / 10) as u8;
                self.memory[self.i + 2] = (val % 10) as u8;
                #[cfg(debug_assertions)]
                eprintln!(
                    "#### {} -> {} {} {}",
                    val,
                    self.memory[self.i],
                    self.memory[self.i + 1],
                    self.memory[self.i + 2]
                );
            }
            Sreg(x) => {
                for r in 0..x + 1 {
                    self.memory[self.i + r] = self.registers[r];
                    //self.i += 1; // QUIRK
                }
            }
            Lreg(x) => {
                for r in 0..x + 1 {
                    self.registers[r] = self.memory[self.i + r];
                    //self.i += 1; // QUIRK
                }
            }
            Err(_) => {
                panic!("Unimplemented instruction {:04X?}!", instr);
            }
        }
    }
}

/// Instructions as enum in an effort to make instruction decoding and execution clearer.
/// Match expressions and doc-comments will make coding easier.
#[derive(Debug)]
enum Instruction {
    /// 0nnn - SYS addr. Jump to machine code at address (unused in practice).
    Sys(usize),
    /// 00E0 - CLS. Clear the screen.
    Cls,
    /// 00EE - RET. Return from subroutine.
    Ret,
    /// 1nnn - JMP addr. Jump to address.
    Jmp(usize),
    /// 2nnn - CALL addr. Call subroutine at address.
    Call(usize),
    /// 3xkk - SKEB Vx, byte. Skip next instruction if VX == byte.
    Skeb(usize, u8),
    /// 4xkk - SKNEB Vx, byte. Skip next instruction if VX != byte.
    Skneb(usize, u8),
    /// 5xy0 - SKE Vx, Vy. Skip next instruction if VX == VY.
    Ske(usize, usize),
    /// 9xy0 - SKNE Vx, Vy. Skip next instruction if VX != VY.
    Skne(usize, usize),
    /// 6xkk - LDB Vx, byte. Load register VX with byte.
    Ldb(usize, u8),
    /// 7xkk - ADDB Vx, byte. Add byte to VX (without overflow status).
    Addb(usize, u8),
    /// 8xy0 - LD Vx, Vy. Load register VX with register VY.
    Ld(usize, usize),
    /// 8xy1 - OR Vx, Vy. Set VX = VX OR VY.
    Or(usize, usize),
    /// 8xy2 - AND Vx, Vy. Set VX = VX AND VY.
    And(usize, usize),
    /// 8xy3 - XOR Vx, Vy. Set VX = VX XOR VY.
    Xor(usize, usize),
    /// 8xy4 - ADD Vx, Vy. Set VX = VX + VY with carry status in VF. Remember that VX can be the same as VF.
    Add(usize, usize),
    /// 8xy5 - SUB Vx, Vy. Set VX = VX - VY with borrow status in VF (not borrow means set). Remember that VX can be the same as VF.
    Sub(usize, usize),
    /// 8xy7 - SUBR Vx, Vy. Set VX = VY - VX with borrow status in VF (not borrow means set). Remember that VX can be the same as VF.
    Subr(usize, usize),
    /// 8xy6 - SHR Vx. Shift VX right with bit 0 before shift in VF. Remember that VX can be the same as VF. Instruction with quirks.
    Shr(usize, usize),
    /// 8xyE - SHL Vx. Shift VX left with bit 7 before shift in VF. Remember that VX can be the same as VF. Instruction with quirks.
    Shl(usize, usize),
    /// Annn - LD I. Set index register to nnn.
    Ldi(usize),
    /// Bnnn - JMPZ addr. Jump to nnn + V0.
    Jmpz(usize),
    /// Cxkk - RND Vx, byte. Set VX to (random number AND byte).
    Rnd(usize, u8),
    /// Dxyn - DRAW Vx, Vy, n. Draw sprite of height n from memory location I at location VX, VY using XOR and collision status in VF (if any bit is flipped from 1 to 0).
    Draw(usize, usize, u8),
    /// Ex9E - SKP Vx. Skip next instruction if key number in VX is pressed.
    Skp(usize),
    /// ExA1 - SKNP Vx. Skip next instruction if key number in VX is not pressed.
    Sknp(usize),
    /// Fx07 - LDFT Vx. Load VX with delay timer value.
    Ldft(usize),
    /// Fx0A - LDKP Vx. Wait for a keypress and load the key num into VX.
    Ldkp(usize),
    /// Fx15 - LDTT Vx. Set delay timer with value from VX.
    Ldtt(usize),
    /// Fx18 - LDST Vx. Set sound timer to value from VX.
    Ldst(usize),
    /// Fx1E - ADDI VX. Set I = I + VX
    Addi(usize),
    /// Fx29 - FONT Vx. Load I with font for key num in VX.
    Font(usize),
    /// Fx33 - BCD Vx. Store BCD value of VX in I, I+1 and I+2.
    Bcd(usize),
    /// Fx55 - SREG Vx. Store registers V0 to VX in memory starting at I.
    Sreg(usize),
    /// Fx65 - LREG Vx. Load register V0 to VX from memory starting at I.
    Lreg(usize),
    /// It's not an instruction. Something's wrong.
    Err(u16),
}

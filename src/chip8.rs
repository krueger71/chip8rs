//! A Chip8 model

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
    /// Sound should play
    pub play_sound: bool,
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
            play_sound: false,
            keyboard: [false; KEYBOARD_SIZE],
        }
    }

    pub fn step(&mut self) {
        let instr = (self.memory[self.pc] as u16) << 8 | (self.memory[1 + self.pc] as u16);
        self.pc += 2;

        // 0xIXYN,0x__NN,0x_NNN
        let i = ((instr & 0xF000) >> 12) as u8;
        let x = ((instr & 0x0F00) >> 8) as usize;
        let y = ((instr & 0x00F0) >> 4) as usize;
        let n = (instr & 0x000F) as u8;
        let nn = (instr & 0x00FF) as u8;
        let nnn = instr & 0x0FFF;

        #[cfg(debug_assertions)]
        eprintln!(
            "instr = {:04x}, i = {:x}, x = {:x}, y = {:x}, n = {:x}, nn = {:02x}, nnn = {:03x}, pc = {:04x}",
            instr, i, x, y, n, nn, nnn, self.pc
        );

        match i {
            0 => {
                match nnn {
                    0x0E0 => {
                        // 00E0 - CLS. Clear the screen
                        self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
                        self.display_update = true;
                    }
                    0x0EE => {
                        // 00EE - RET. Return from subroutine
                        self.pc = self.stack[self.sp];
                        self.sp -= 1;
                    }
                    _ => {
                        unmatched_instruction(instr);
                    }
                }
            }
            1 => {
                // 1NNN - JP. Jump to address NNN
                self.pc = nnn as usize;
            }
            2 => {
                // 2NNN - CALL. Call subroutine at address NNN
                self.sp += 1;
                self.stack[self.sp] = self.pc;
                self.pc = nnn as usize;
            }
            3 => {
                // 3XNN - SE VX, byte. Skip next instruction if VX == NN
                if self.registers[x] == nn {
                    self.pc += 2;
                }
            }
            4 => {
                // 4XNN - SNE VX, byte. Skip next instruction if VX != NN
                if self.registers[x] != nn {
                    self.pc += 2;
                }
            }
            5 => {
                // 5XY0 - SE VX, VY. Skip next instruction if VX == VY
                match n {
                    0 => {
                        if self.registers[x] == self.registers[y] {
                            self.pc += 2;
                        }
                    }
                    _ => {
                        unmatched_instruction(instr);
                    }
                }
            }
            6 => {
                // 6XNN - LD VX, byte. Set register VX to NN
                self.registers[x] = nn;
            }
            7 => {
                // 7XNN - ADD VX, byte. Add NN to register VX
                self.registers[x] = (self.registers[x] as u16 + nn as u16) as u8;
            }
            8 => {
                match n {
                    0 => {
                        // 8XY0 - LD VX, VY. Store value of VY in VX
                        self.registers[x] = self.registers[y];
                    }
                    1 => {
                        // 8XY1 - OR VX, VY. Store value of VX OR VY in VX
                        self.registers[x] |= self.registers[y];
                    }
                    2 => {
                        // 8XY2 - AND VX, VY. Store value of VX AND VY in VX
                        self.registers[x] &= self.registers[y];
                    }
                    3 => {
                        // 8XY3 - XOR VX, VY. Store value of VX XOR VY in VX
                        self.registers[x] ^= self.registers[y];
                    }
                    4 => {
                        // 8XY4 - ADD VX, VY. Store value of VX + VY in VX with overflow status in VF
                        let (result, overflow) =
                            self.registers[x].overflowing_add(self.registers[y]);
                        self.registers[x] = result;

                        if overflow {
                            self.registers[0xF] = 1;
                        } else {
                            self.registers[0xF] = 0;
                        }
                    }
                    5 => {
                        // 8XY5 - SUB VX, VY. Store value of VX - VY in VX with overflow status in VF
                        let (result, overflow) =
                            self.registers[x].overflowing_sub(self.registers[y]);
                        self.registers[x] = result;

                        if overflow {
                            self.registers[0xF] = 0;
                        } else {
                            self.registers[0xF] = 1;
                        }
                    }
                    6 => {
                        // 8XY6 - SHR VX. Set VF to LSB and shift value in VX right one bit
                        self.registers[0xF] = self.registers[x] & 1;
                        self.registers[x] >>= 1;
                    }
                    7 => {
                        // 8XY7 - SUBN VX, VY. Store value of VY - VX in VX with overflow status in VF
                        let (result, overflow) =
                            self.registers[y].overflowing_sub(self.registers[x]);
                        self.registers[x] = result;

                        if overflow {
                            self.registers[0xF] = 0;
                        } else {
                            self.registers[0xF] = 1;
                        }
                    }
                    0xE => {
                        // 8XYE - SHL VX. Set VF to MSB and shift value in VX left one bit
                        self.registers[0xF] = self.registers[x] & 0b1000_0000;
                        self.registers[x] <<= 1;
                    }
                    _ => {
                        unmatched_instruction(instr);
                    }
                }
            }
            9 => {
                // 9XY0 - SNE VX, VY. Skip next instruction if VX != VY
                match n {
                    0 => {
                        if self.registers[x] != self.registers[y] {
                            self.pc += 2;
                        }
                    }
                    _ => {
                        unmatched_instruction(instr);
                    }
                }
            }
            0xA => {
                // ANNN - LD I, addr. Set index register I to address
                self.i = nnn as usize;
            }
            0xB => {
                // BNNN - JP V0, addr. Jump to location nnn + V0
                self.pc = (nnn + self.registers[0] as u16) as usize;
            }
            0xC => {
                // CXNN - RND Vx, byte. Set VX to random number AND NN
                self.registers[x] = rand::random::<u8>() & nn;
            }
            0xD => {
                // DXYN - DRW VX, VY, nibble. Draw n-byte sprite at X,Y with collision detection using XOR
                let px = (self.registers[x] % (DISPLAY_WIDTH as u8)) as usize;
                let py = (self.registers[y] % (DISPLAY_HEIGHT as u8)) as usize;
                let idx = self.i as usize;
                let sprite = &self.memory[idx..(idx + n as usize)];
                self.registers[0xF] = 0;

                // Iterate over each individual bit in each byte of sprite
                // Set each bit according to the rules for DXYN draw in display
                // Sprites wrap-around immediately in this implementation, which is probably incorrect

                for (dy, byte) in sprite.iter().enumerate() {
                    for dx in 0..8 {
                        let old =
                            self.display[(py + dy) % DISPLAY_HEIGHT][(px + dx) % DISPLAY_WIDTH];
                        let mut new = ((byte >> (7 - dx)) & 1) == 1;

                        if new {
                            if old {
                                new = false;
                                self.registers[0xF] = 1;
                            }

                            self.display[(py + dy) % DISPLAY_HEIGHT][(px + dx) % DISPLAY_WIDTH] =
                                new;
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
            0xE => {
                match nn {
                    0x9E => {
                        //  Ex9E - SKP Vx
                        if self.keyboard[self.registers[x] as usize] {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        // ExA1 - SKNP Vx
                        if !self.keyboard[self.registers[x] as usize] {
                            self.pc += 2;
                        }
                    }
                    _ => {
                        unmatched_instruction(instr);
                    }
                }
            }
            0xF => {
                match nn {
                    0x07 => {
                        // Fx07 - LD Vx, DT
                        self.registers[x] = self.dt;
                    }
                    0x0A => {
                        //  Fx0A - LD Vx, K
                        for (key, pressed) in self.keyboard.iter().enumerate() {
                            if *pressed {
                                self.registers[x] = key as u8;
                                break;
                            } else {
                                self.pc -= 2; // Undo increment to next instruction (repeat this instruction)
                            }
                        }
                    }
                    0x15 => {
                        // Fx15 - LD DT, Vx
                        self.dt = self.registers[x];
                    }
                    0x18 => {
                        // Fx18 - LD ST, Vx
                        self.st = self.registers[x];
                    }
                    0x1E => {
                        //  Fx1E - ADD I, Vx
                        self.i += self.registers[x] as usize;
                    }
                    0x29 => {
                        //  Fx29 - LD F, Vx
                        self.i = (self.registers[x] * 5) as usize; // Fonts are loaded at address 0x0 and are 5 bytes in size
                    }
                    0x33 => {
                        // Fx33 - LD B, Vx
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
                    0x55 => {
                        // Fx55 - LD [I], Vx. Store regs in memory
                        for r in 0..x + 1 {
                            self.memory[self.i + r] = self.registers[r];
                        }
                    }
                    0x65 => {
                        // Fx65 - LD Vx, [I]. Load regs from memory
                        for r in 0..x + 1 {
                            self.registers[r] = self.memory[self.i + r];
                        }
                    }
                    _ => {
                        unmatched_instruction(instr);
                    }
                }
            }
            _ => {
                unmatched_instruction(instr);
            }
        }
        //println!("pc = {:04x}", self.pc);
    }
}

/// Handle instruction not decoded
fn unmatched_instruction(instr: u16) {
    panic!("instr = {:04x} not decoded!", instr);
}

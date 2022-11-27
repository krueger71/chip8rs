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
    pub i: u16,
    /// Program counter
    pub pc: u16,
    /// Stack pointer
    pub sp: u8,
    /// Stack
    pub stack: [u16; STACK_SIZE],

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
            pc: PROGRAM_START as u16,
            sp: 0,
            stack: [0; STACK_SIZE],
            display: [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT],
            display_update: false,
            play_sound: false,
            keyboard: [false; KEYBOARD_SIZE],
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

        #[cfg(debug_assertions)]
        eprintln!(
            "instr = {:04x}, i = {:x}, x = {:x}, y = {:x}, n = {:x}, nn = {:02x}, nnn = {:03x}, pc = {:04x}",
            instr, i, x, y, n, nn, nnn, self.pc
        );

        match i {
            0 => {
                match nnn {
                    0x0e0 => {
                        // 00E0 - CLS. Clear the screen
                        self.display = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];
                        self.display_update = true;
                    }
                    0x0ee => {
                        // 00EE - RET. Return from subroutine
                        self.pc = self.stack[self.sp as usize];
                        self.sp -= 1;
                    }
                    _ => {
                        unmatched_instruction(instr);
                    }
                }
            }
            1 => {
                // 1NNN - JP. Jump to address NNN
                self.pc = nnn;
            }
            2 => {
                // 2NNN - CALL. Call subroutine at address NNN
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
            }
            3 => {
                // 3XNN - SE VX, byte. Skip next instruction if VX == NN
                if self.registers[x as usize] == nn {
                    self.pc += 2;
                }
            }
            4 => {
                // 4XNN - SNE VX, byte. Skip next instruction if VX != NN
                if self.registers[x as usize] != nn {
                    self.pc += 2;
                }
            }
            5 => {
                // 5XY0 - SE VX, VY. Skip next instruction if VX == VY
                match n {
                    0 => {
                        if self.registers[x as usize] == self.registers[y as usize] {
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
                self.registers[x as usize] = nn;
            }
            7 => {
                // 7XNN - ADD VX, byte. Add NN to register VX
                self.registers[x as usize] = (self.registers[x as usize] as u16 + nn as u16) as u8;
            }
            8 => {
                match n {
                    0 => {
                        // 8XY0 - LD VX, VY. Store value of VY in VX
                        self.registers[x as usize] = self.registers[y as usize];
                    }
                    1 => {
                        // 8XY1 - OR VX, VY. Store value of VX OR VY in VX
                        self.registers[x as usize] |= self.registers[y as usize];
                    }
                    2 => {
                        // 8XY2 - AND VX, VY. Store value of VX AND VY in VX
                        self.registers[x as usize] &= self.registers[y as usize];
                    }
                    3 => {
                        // 8XY3 - XOR VX, VY. Store value of VX XOR VY in VX
                        self.registers[x as usize] ^= self.registers[y as usize];
                    }
                    4 => {
                        // 8XY4 - ADD VX, VY. Store value of VX + VY in VX with overflow status in VF
                        let (result, overflow) =
                            self.registers[x as usize].overflowing_add(self.registers[y as usize]);
                        self.registers[x as usize] = result;

                        if overflow {
                            self.registers[0xf] = 1;
                        } else {
                            self.registers[0xf] = 0;
                        }
                    }
                    5 => {
                        // 8XY5 - SUB VX, VY. Store value of VX - VY in VX with overflow status in VF
                        let (result, overflow) =
                            self.registers[x as usize].overflowing_sub(self.registers[y as usize]);
                        self.registers[x as usize] = result;

                        if overflow {
                            self.registers[0xf] = 0;
                        } else {
                            self.registers[0xf] = 1;
                        }
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
                        if self.registers[x as usize] != self.registers[y as usize] {
                            self.pc += 2;
                        }
                    }
                    _ => {
                        unmatched_instruction(instr);
                    }
                }
            }
            0xA => {
                // ANNN - LD I, address. Set index register I to address
                self.i = nnn;
            }
            0xD => {
                // DXYN - DRW VX, VY, nibble. Draw n-byte sprite at X,Y with collision detection using XOR
                let px = self.registers[x as usize] % (DISPLAY_WIDTH as u8);
                let py = self.registers[y as usize] % (DISPLAY_HEIGHT as u8);
                let idx = self.i as usize;
                let sprite = &self.memory[idx..(idx + n as usize)];
                self.registers[0xf] = 0;

                // Iterate over each individual bit in each byte of sprite
                // Set each bit according to the rules for DXYN draw in display
                // Sprites wrap-around immediately in this implementation, which is probably incorrect

                for (dy, byte) in sprite.iter().enumerate() {
                    for dx in 0..8 {
                        let old = self.display[(py as usize + dy as usize) % DISPLAY_HEIGHT]
                            [(px as usize + dx as usize) % DISPLAY_WIDTH];
                        let mut new = ((byte >> (7 - dx)) & 1) == 1;

                        if new {
                            if old {
                                new = false;
                                self.registers[0xf] = 1;
                            }
                            self.display[(py as usize + dy as usize) % DISPLAY_HEIGHT]
                                [(px as usize + dx as usize) % DISPLAY_WIDTH] = new;
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
            _ => {
                unmatched_instruction(instr);
            }
        }

        //println!("pc = {:04x}", self.pc);
    }
}

/// Handle instruction not decoded
fn unmatched_instruction(instr: u16) {
    eprintln!("instr = {:04x} not decoded!", instr);
}

use std::fs::File;
use std::io::prelude::*;

const CHIP8_RAM: usize = 4096;
const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

pub struct VM {
    ram: [u8; CHIP8_RAM],
    vram: [[u8; CHIP8_WIDTH]; CHIP8_HEIGHT],  // graphics memory
    vram_changed: bool,
    stack: [usize; 16],
    v: [u8; 16],  // cpu registers
    i: u16,
    pc: u16,
    sp: u16,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; 16],
    keypad_waiting: usize, // ?
    keypad_register: usize, // ?
}

impl VM {
    pub fn new() -> Self {
        let ram = [0; CHIP8_RAM];

        Self {
            vram: [[0; CHIP8_WIDTH]; CHIP8_HEIGHT],
            vram_changed: false,
            ram: ram,
            v: [0; 16],
            stack: [0; 16],
            i: 0,
            pc: 0x200,
            sp: 0,
            keypad: [false; 16],
            keypad_waiting: 0,
            keypad_register: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            let addr = 0x200 + i;
            if addr >= 4096 {
                break
            }
            self.ram[addr] = byte;
        }
    }
}

fn load_rom(path: &str, buf: &mut [u8; 4096]) -> usize {
    let mut f = File::open(path).expect("file not found");
    match f.read(buf) {
        Err(_) => 0,
        Ok(n) => n,
    }
}

fn main() {
    let mut buf = [0u8; 4096];
    load_rom("games/Airplane.ch8", &mut buf);
    let mut vm = VM::new();
    vm.load(&buf);
}

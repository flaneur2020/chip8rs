const CHIP8_RAM: usize = 4096;
const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;


pub struct Chip8Machine {
    ram: [u8, CHIP8_RAM],
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


impl Chip8Machine {
    pub fn new() -> Self {
        let mut ram = [0u8; CHIP8_RAM];

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
            keypad_waiting: false,
            keypad_register: 0,
        }
    }

    pub fn load(&mut self, data: &[u8]) {
    }
}


fn main() {
    println!("Hello, world!");
}

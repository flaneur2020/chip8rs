extern crate rand;

mod vm;
mod font;
mod instruction;
mod ui;

use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;


fn load_rom(path: &str, buf: &mut [u8; 4096]) -> usize {
    let mut f = File::open(path).expect("file not found");
    match f.read(buf) {
        Err(_) => 0,
        Ok(n) => n,
    }
}

fn main() {
    let mut buf = [0u8; 4096];
    let mut vm = vm::VM::new();
    let sdl_context = sdl2::init().unwrap();
    let mut ui = ui::UI::new(sdl_context);

    load_rom("games/Airplane.ch8", &mut buf);
    vm.load(&buf);

    while let Ok(keypad) = ui.poll() {
        let output = vm.step(keypad);
        if output.vram_changed {
            ui.draw(output.vram);
        }

        // let mut pixels = [[0u8; vm::CHIP8_WIDTH]; vm::CHIP8_HEIGHT];
        // pixels[0][0] = 1;
        // ui.draw(&pixels);

        thread::sleep(Duration::from_millis(2));
    }
}
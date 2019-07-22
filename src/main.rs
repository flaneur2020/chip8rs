extern crate rand;

use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
mod chip8rs;


fn load_rom(path: &str, buf: &mut [u8; 4096]) -> usize {
    let mut f = File::open(path).expect("file not found");
    match f.read(buf) {
        Err(_) => 0,
        Ok(n) => n,
    }
}

fn main() {
    let mut buf = [0u8; 4096];
    let sdl_context = sdl2::init().unwrap();
    let mut vm = chip8rs::VM::new();
    let mut ui = chip8rs::UI::new(sdl_context);

    load_rom("games/Landing.ch8", &mut buf);
    vm.load(&buf);

    while let Ok(keypad) = ui.poll() {
        let output = vm.step(keypad);
        if output.vram_changed {
            ui.draw(output.vram);
        }

        thread::sleep(Duration::from_micros(2000));
    }
}

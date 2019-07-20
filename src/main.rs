extern crate rand;

use std::fs::File;
use std::io::prelude::*;

mod vm;
mod font;
mod instruction;

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
    let mut vm = vm::VM::new();
    vm.load(&buf);
}
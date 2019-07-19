pub type Register = u8;
pub type Addr = u16;
pub type Byte = u8;

#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
pub enum Instruction {
    // 00E0 - CLS, clear the screen
    Clear,
    // 00EE - RET
    Return,
    // 0nnn - SYS addr
    Sys(Addr),
    // 1nnn - JP addr
    Jump(Addr),
    // 2nnn
    Call(Addr),
    // 3xkk - SE Vx, byte: Skips the next instructions if `Vx` equals `Byte`
    SkipEqualK(Register, Byte),
    // 4xkk - SNE Vx, byte
    SkipNotEqualK(Register, Byte),
    // 5xy0 - SE Vx, Vy: Skips the next instruction if `Vx` and `Vy` are equal
    SkipEqual(Register, Register),
    // 6xkk - LD Vx, byte: Sets `Vy` to `Byte`
    LoadK(Register, Byte),
    // 7xkk - LD Vx, byte: Sets `Vy` to `Byte`
    AddK(Register, Byte),
    // 8xy0 - LD Vx, Vy: Stores `Vy` in `Vx`
    Set(Register, Register),
    // 8xy1 - OR Vx, Vy: Performs a bitwise OR (`|`) of `Vx` and `Vy`, then stores the result in `Vx`
    Or(Register, Register),
    // 8xy2 - AND Vx, Vy: Performs a bitwise AND (`&`) of `Vx` and `Vy`, then stores the result in `Vx`
    And(Register, Register),
    // 8xy3 - XOR Vx, Vy: Performs a bitwise XOR (`^`) of `Vx` and `Vy`, then stores the result in `Vx`
    Xor(Register, Register),
    // 8xy4 - ADD Vx, Vy: Adds `Vx` and `Vy`, then stores the result in in `Vx`, `VF` is set to `1` on overflow, `0` otherwise.
    Add(Register, Register),
    // 8xy5 - SUB Vx, Vy: Subtracts `Vy` from `Vx`, then stores the result in `Vx`, `VF` is set to `1` if `Vx` is larger than `Vy` prior subtraction, `0` otherwise.
    Sub(Register, Register),
    // 8xy6 - SHR Vx {, Vy}: Shifts `Vy` right by one bit, then stores the result in `Vx`. Stores the least-significant bit prior shift of `Vy` in `VF`.
    ShiftRight(Register),
    // 8xy7 - SUBN Vx, Vy: Subtracts `Vx` from `Vy`, then stores the result in `Vx`. `VF` is set to `1` if `Vx` is larger than `Vy` prior subtraction, `0` otherwise. Note that this is the same as `Sub` with inverted register operands.
    SubInv(Register, Register),
    // 8xyE - SHL Vx {, Vy}: Shifts `Vy` left by one bit, then stores the result in `Vx`. Stores the most-significant bit prior shift of `Vy` in `VF`.
    ShiftLeft(Register),
    // 9xy0 - SNE Vx, Vy: Skips the next instruction if `Vx` and `Vy` are not equal
    SkipNotEqual(Register, Register),
    // Annn - LD I, addr: Sets the `I` register to `Addr`
    LoadI(Addr),
    // Bnnn - JP V0, addr: Jumps to `V0 + Addr`
    LongJump(Addr),
    // Cxkk - RND Vx, byte: Sets `Vx` to a random byte ANDed with `Byte`
    Rand(Register, Byte),
    // Dxyn - DRW Vx, Vy, nibble: Draws the sprite with `Nibble` bytes of data from the `I` register at position `(Vx, Vy)`. Sets `VF` to `1` if any pixels are set to unlit state, `0` otherwise. Note that sprites wrap around onto the opposite side of the screen.
    Draw(Register, Register, Byte),
    // Ex9E - SKP Vx: Skips the next instruction if key `Vx` is pressed
    SkipPressed(Register),
    // ExA1 - SKNP Vx: Skips the next instruction if key `Vx` is not pressed
    SkipNotPressed(Register),
    // Fx07 - LD Vx, DT: Stores the value of the `delay timer` in `Vx`
    GetTimer(Register),
    // Fx0A - LD Vx, K: Stops execution until a key is pressed, then stores that key in `Vx`
    WaitKey(Register),
    // Fx15 - LD DT, Vx: Sets the `delay timer` to `Vx`
    SetTimer(Register),
    // Fx18 - LD ST, Vx: Sets the `sound timer` to `Vx`
    SetSoundTimer(Register),
    // Fx1E - ADD I, Vx: Adds `Vx` and the `I` register, then stores the result in `I`
    AddI(Register),
    // Fx29 - LD F, Vx: Stores the address of the hexadecimal digit `Vx` in the `I` register
    LoadHexGlyph(Register),
    // Fx33 - LD B, Vx: Stores the binary-coded decimal representation of `Vx` at address `I`, `I + 1` and `I + 2`
    StoreBCD(Register),
    // Fx55 - LD [I], Vx: Stores the registers `V0` to `Vx` inclusive at address `I`. Register `I` is set to `I + Vx + 1` afterwards.
    StoreRegisters(Register),
    // Fx65 - LD Vx, [I]: Reads the registers `V0` to `Vx` inclusive from address `I`. Register `I` is set to `I + Vx + 1` afterwards.
    LoadRegisters(Register),
    // Placeholder for an unknown or illegal instruction.
    Unknown,
}

impl Instruction {
    pub fn decode(opcode: u16) -> Instruction {
        let nibbles = (
            (opcode & 0xF000) >> 12 as u8,
            (opcode & 0x0F00) >> 8 as u8,
            (opcode & 0x00F0) >> 4 as u8,
            (opcode & 0x000F) as u8,
        );

        let nnn = (opcode & 0x0FFF) as usize;
        let kk = (opcode & 0x00FF) as Byte;
        let x = nibbles.1 as Register;
        let y = nibbles.2 as Register;
        let n = nibbles.3 as Byte;

        match nibbles {
            (0x00, 0x00, 0x0e, 0x00) => Instruction::Clear,
            (0x00, 0x00, 0x0e, 0x0e) => Instruction::Return,
            (0x01, _, _, _) => Instruction::Jump(nnn as Addr),
            (0x02, _, _, _) => Instruction::Call(nnn as Addr),
            (0x03, _, _, _) => Instruction::SkipEqualK(x, kk),
            (0x04, _, _, _) => Instruction::SkipNotEqualK(x, kk),
            (0x05, _, _, 0x00) => Instruction::SkipEqual(x, y),
            (0x06, _, _, _) => Instruction::LoadK(x, kk),
            (0x07, _, _, _) => Instruction::AddK(x, kk),
            (0x08, _, _, 0x00) => Instruction::Set(x, y),
            (0x08, _, _, 0x01) => Instruction::Or(x, y),
            (0x08, _, _, 0x02) => Instruction::And(x, y),
            (0x08, _, _, 0x03) => Instruction::Xor(x, y),
            (0x08, _, _, 0x04) => Instruction::Add(x, y),
            (0x08, _, _, 0x05) => Instruction::Sub(x, y),
            (0x08, _, _, 0x06) => Instruction::ShiftRight(x),
            (0x08, _, _, 0x07) => Instruction::SubInv(x, y),
            (0x08, _, _, 0x0e) => Instruction::ShiftLeft(x),
            (0x09, _, _, 0x00) => Instruction::SkipNotEqual(x, y),
            (0x0a, _, _, _) => Instruction::LoadI(nnn as Addr),
            (0x0b, _, _, _) => Instruction::LongJump(nnn as Addr),
            (0x0c, _, _, _) => Instruction::Rand(x, kk),
            (0x0d, _, _, _) => Instruction::Draw(x, y, n),
            (0x0e, _, 0x09, 0x0e) => Instruction::SkipPressed(x),
            (0x0e, _, 0x0a, 0x01) => Instruction::SkipNotPressed(x),
            (0x0f, _, 0x00, 0x07) => Instruction::GetTimer(x),
            (0x0f, _, 0x00, 0x0A) => Instruction::WaitKey(x),
            (0x0f, _, 0x01, 0x05) => Instruction::SetTimer(x),
            (0x0f, _, 0x01, 0x08) => Instruction::SetSoundTimer(x),
            (0x0f, _, 0x01, 0x0e) => Instruction::AddI(x),
            (0x0f, _, 0x02, 0x09) => Instruction::LoadHexGlyph(x),
            (0x0f, _, 0x03, 0x03) => Instruction::StoreBCD(x),
            (0x0f, _, 0x05, 0x05) => Instruction::StoreRegisters(x),
            (0x0f, _, 0x06, 0x05) => Instruction::LoadRegisters(x),
            _ => Instruction::Unknown,
        }
    }
}

#[cfg(test)]
#[path = "./instruction_test.rs"]
mod instruction_test;
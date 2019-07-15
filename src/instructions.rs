pub type Register = u8;
pub type Addr = u16;

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
    SkipEqualK(Register, u8),
    // 4xkk - SNE Vx, byte
    SkipNotEqualK(Register, u8),
    // 5xy0 - SE Vx, Vy: Skips the next instruction if `Vx` and `Vy` are equal
    SkipEqual(Register, Register),
    // 6xkk - LD Vx, byte: Sets `Vy` to `Byte`
    SetK(Register, u8),
    // 7xkk - LD Vx, byte: Sets `Vy` to `Byte`
    AddK(Register, u8),
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
    ShiftRight(Register, Register),
    // 8xy7 - SUBN Vx, Vy: Subtracts `Vx` from `Vy`, then stores the result in `Vx`. `VF` is set to `1` if `Vx` is larger than `Vy` prior subtraction, `0` otherwise. Note that this is the same as `Sub` with inverted register operands.
    SubInv(Register, Register),
    // 8xyE - SHL Vx {, Vy}: Shifts `Vy` left by one bit, then stores the result in `Vx`. Stores the most-significant bit prior shift of `Vy` in `VF`.
    ShiftLeft(Register, Register),
    // 9xy0 - SNE Vx, Vy: Skips the next instruction if `Vx` and `Vy` are not equal
    SkipNotEqual(Register, Register),
    // Annn - LD I, addr: Sets the `I` register to `Addr`
    LoadI(Addr),
    // Bnnn - JP V0, addr: Jumps to `V0 + Addr`
    LongJump(Addr),
    // Cxkk - RND Vx, byte: Sets `Vx` to a random byte ANDed with `Byte`
    Rand(Register, u8),
    // Dxyn - DRW Vx, Vy, nibble: Draws the sprite with `Nibble` bytes of data from the `I` register at position `(Vx, Vy)`. Sets `VF` to `1` if any pixels are set to unlit state, `0` otherwise. Note that sprites wrap around onto the opposite side of the screen.
    Draw(Register, Register, u8),
    // Ex9E - SKP Vx: Skips the next instruction if key `Vx` is pressed
    SkipPressed(Register),
    // ExA1 - SKNP Vx: Skips the next instruction if key `Vx` is not pressed
    SkipNotPressed(Register),
    // Fx07 - LD Vx, DT: Stores the value of the `delay timer` in `Vx`
    GetTimer(Register),
    // Fx0A - LD Vx, K: Stops execution until a key is pressed, then stores that key in `Vx`
    WaitKey(Register),
    // Fx15 - LD DT, Vx: Sets the `delay timer` to `Vx`
    SetTiemr(Register),
    // Fx18 - LD ST, Vx: Sets the `sound timer` to `Vx`
    SetSoundTimer(Register),
    // Fx1E - ADD I, Vx: Adds `Vx` and the `I` register, then stores the result in `I`
    AddToI(Register),
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
    pub fn decode(code: u32) -> Instruction {
        return Instruction::Unknown;
    }
}
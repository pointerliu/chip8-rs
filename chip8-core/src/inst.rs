use crate::error::Chip8Error;

pub enum Instruction {
    I0NNN(u16),
    I00E0,
    I00EE,
    I1NNN(u16),
    I2NNN(u16),
    I3XKK(u8, u8),
    I4XKK(u8, u8),
    I5XY0(u8, u8),
    I6XKK(u8, u8),
    I7XKK(u8, u8),
    I8XY0(u8, u8),
    I8XY1(u8, u8),
    I8XY2(u8, u8),
    I8XY3(u8, u8),
    I8XY4(u8, u8),
    I8XY5(u8, u8),
    I8XY6(u8, u8),
    I8XY7(u8, u8),
    I8XYE(u8, u8),
    I9XY0(u8, u8),
    IANNN(u16),
    IBNNN(u16),
    ICXKK(u8, u8),
    IDXYN(u8, u8, u8),
    IEX9E(u8),
    IEXA1(u8),
    IFX07(u8),
    IFX0A(u8),
    IFX15(u8),
    IFX18(u8),
    IFX1E(u8),
    IFX29(u8),
    IFX33(u8),
    IFX55(u8),
    IFX65(u8)
}

impl Instruction {
    fn get_nibbles(value: u16) -> (u8, u8, u8, u8) {
        (
            ((value & 0xF000) >> 12) as u8,
            ((value & 0x0F00) >> 8) as u8,
            ((value & 0x00F0) >> 4) as u8,
            (value & 0x00F) as u8,
        )
    }

    fn get_nnn(value: u16) -> u16 {
        value & 0x0FFF
    }

    fn get_x(value: u16) -> u8 {
        ((value & 0x0F00) >> 8) as u8
    }

    fn get_y(value: u16) -> u8 {
        ((value & 0x00F0) >> 4) as u8
    }

    fn get_kk(value: u16) -> u8 {
        (value & 0x00FF) as u8
    }

    fn get_n(value: u16) -> u8 {
        (value & 0x000F) as u8
    }
}

impl TryFrom<u16> for Instruction {
    type Error = Chip8Error;
    fn try_from(value: u16) -> Result<Self, Chip8Error> {
        let (n1, n2, n3, n4) = Self::get_nibbles(value);
        let nnn = Self::get_nnn(value);
        let x = Self::get_x(value);
        let y = Self::get_y(value);
        let n = Self::get_n(value);
        let kk = Self::get_kk(value);
        match n1 {
            0 => match n4 {
                0xE => Ok(Instruction::I00EE),
                0x0 => Ok(Instruction::I00E0),
                _ => Err(Chip8Error::DecodeError("Unknown instruction".into(), value))
            },
            1 => Ok(Instruction::I1NNN(nnn)),
            2 => Ok(Instruction::I2NNN(nnn)),
            3 => Ok(Instruction::I3XKK(x, kk)),
            4 => Ok(Instruction::I4XKK(x, kk)),
            5 => Ok(Instruction::I5XY0(x, y)),
            6 => Ok(Instruction::I6XKK(x, kk)),
            7 => Ok(Instruction::I7XKK(x, kk)),
            8 => match n4 {
                0 => Ok(Instruction::I8XY0(x, y)),
                1 => Ok(Instruction::I8XY1(x, y)),
                2 => Ok(Instruction::I8XY2(x, y)),
                3 => Ok(Instruction::I8XY3(x, y)),
                4 => Ok(Instruction::I8XY4(x, y)),
                5 => Ok(Instruction::I8XY5(x, y)),
                6 => Ok(Instruction::I8XY6(x, y)),
                7 => Ok(Instruction::I8XY7(x, y)),
                0xE => Ok(Instruction::I8XYE(x, y)),
                _ => Err(Chip8Error::DecodeError("Unknown instruction".into(), value))
            },
            9 => Ok(Instruction::I9XY0(x, y)),
            0xA => Ok(Instruction::IANNN(nnn)),
            0xB => Ok(Instruction::IBNNN(nnn)),
            0xC => Ok(Instruction::ICXKK(x, kk)),
            0xD => Ok(Instruction::IDXYN(x, y, n)),
            0xE => match (n2, n3) {
                (9, 0xE) => Ok(Instruction::IEX9E(x)), 
                (0xA, 1) => Ok(Instruction::IEXA1(x)), 
                _ => Err(Chip8Error::DecodeError("Unknown instruction".into(), value))
            },
            0xF => match (n3, n4) {
                (0, 7) => Ok(Instruction::IFX07(x)),
                (0, 0xA) => Ok(Instruction::IFX0A(x)),
                (1, 5) => Ok(Instruction::IFX15(x)),
                (1, 8) => Ok(Instruction::IFX18(x)),
                (1, 0xE) => Ok(Instruction::IFX1E(x)),
                (2, 9) => Ok(Instruction::IFX29(x)),
                (3, 3) => Ok(Instruction::IFX33(x)),
                (5, 5) => Ok(Instruction::IFX55(x)),
                (6, 5) => Ok(Instruction::IFX65(x)),
                _ => Err(Chip8Error::DecodeError("Unknown instruction".into(), value))
            },
            _ => Err(Chip8Error::DecodeError("Unknown instruction".into(), value))
        }
    }
}

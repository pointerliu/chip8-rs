use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum Chip8Error {
    DecodeError(String, u16),
}

impl Display for Chip8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for Chip8Error {}

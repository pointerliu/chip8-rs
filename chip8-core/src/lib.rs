mod chip8;
mod consts;
mod error;
mod inst;

pub use chip8::Chip8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

mod consts;
mod chip8;
mod inst;
mod error;

pub use chip8::Chip8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}

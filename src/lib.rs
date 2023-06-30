#![no_std]

mod mt19937;
pub use mt19937::MT19937;
pub use mt19937::DEFAULT_SEED;
pub use mt19937::DEFAULT_SEED_PS2;
pub use mt19937::MTState;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

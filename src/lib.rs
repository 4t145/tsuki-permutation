mod permutation;
pub use permutation::*;
mod alternating;
pub use alternating::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Parity {
    Even = 0,
    Odd = 1,
}

impl Parity {
    pub fn is_even(self) -> bool {
        self == Parity::Even
    }
    pub fn is_odd(self) -> bool {
        self == Parity::Odd
    }
    pub fn flip(self) -> Self {
        match self {
            Parity::Even => Parity::Odd,
            Parity::Odd => Parity::Even,
        }
    }
}
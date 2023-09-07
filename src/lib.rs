mod permutation;
pub use permutation::*;
mod alternating;
pub use alternating::*;
mod cyclic;
pub use cyclic::*;
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

pub trait Group {
    fn unit() -> Self;
    fn inverse(&self) -> Self;
    fn op(&self, rhs: Self) -> Self;
}

pub struct DirectProduct<G, H>(G, H);

impl<G, H> Group for DirectProduct<G, H>
where
    G: Group,
    H: Group,
{
    fn unit() -> Self {
        Self(G::unit(), H::unit())
    }
    fn inverse(&self) -> Self {
        Self(self.0.inverse(), self.1.inverse())
    }
    fn op(&self, rhs: Self) -> Self {
        Self(self.0.op(rhs.0), self.1.op(rhs.1))
    }
}




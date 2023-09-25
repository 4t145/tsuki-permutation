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

pub trait Group: Clone {
    fn unit() -> Self;
    fn inverse(&self) -> Self;
    fn op(&self, rhs: &Self) -> Self;
}

pub fn commutator<G: Group>(a: &G, b: &G) -> G {
    a.inverse().op(&b.inverse()).op(a).op(b)
}

pub fn conjugate<G: Group>(a: &G, b: &G) -> G {
    b.inverse().op(a).op(b)
}

pub trait FiniteGroup: Group {
    fn order() -> usize;
    fn enumerate() -> Box<dyn Iterator<Item = Self>>
    where
        Self: Sized;
}

pub trait Action {
    type G: Group;
    type X;
    fn act(g: &Self::G, x: Self::X) -> Self::X;
}
pub struct Orbit<A: Action>(A::X);
impl<A: Action> Clone for Orbit<A>
where
    A::X: Clone,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<A: Action> Orbit<A> {
    pub fn on(&self, g: &A::G) -> A::X
    where
        A::X: Clone,
    {
        A::act(g, self.0.clone())
    }
}
#[derive(Clone)]
pub struct Commutator<G>(G, G);
impl<G: Group> Commutator<G> {
    pub fn new(a: G, b: G) -> Self {
        Self(a, b)
    }
    pub fn eval(&self) -> G {
        commutator(&self.0, &self.1)
    }
}

impl<G: Group> Group for Commutator<G> {
    fn unit() -> Self {
        Self(G::unit(), G::unit())
    }
    fn inverse(&self) -> Self {
        Self(self.1.clone(), self.0.clone())
    }
    fn op(&self, rhs: &Self) -> Self {
        Self(self.0.op(&rhs.0), self.1.op(&rhs.1))
    }
}

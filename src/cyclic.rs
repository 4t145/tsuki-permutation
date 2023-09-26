use crate::Group;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Cyclic<const N: usize>(usize);

impl<const N: usize> Cyclic<N> {
    pub fn new(n: usize) -> Option<Self> {
        (n < N).then_some(Self(n))
    }

    /// # Safety
    /// n must be less than N.
    pub const unsafe fn new_unchecked(n: usize) -> Self {
        Self(n)
    }

    pub const fn unit() -> Self {
        Self(0)
    }
    pub const fn inverse(&self) -> Self {
        Self(N - self.0)
    }
    pub const fn compose(&self, other: &Self) -> Self {
        Self((self.0 + other.0) % N)
    }
}

impl<const N: usize> Group for Cyclic<N> {
    fn unit() -> Self {
        Self::unit()
    }
    fn inverse(&self) -> Self {
        self.inverse()
    }
    fn op(&self, rhs: &Self) -> Self {
        self.compose(rhs)
    }
}

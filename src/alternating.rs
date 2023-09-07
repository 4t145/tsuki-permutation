use crate::{Permutation, Group};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Alternating<const N: usize>(Permutation<N>);

impl<const N: usize> Alternating<N> {
    pub fn new(perm: Permutation<N>) -> Option<Self> {
        perm.parity()
            .is_even()
            .then_some(unsafe { Self::new_unchecked(perm) })
    }
    /// # Safety
    /// perm must be an even permutation.
    pub unsafe fn new_unchecked(perm: Permutation<N>) -> Self {
        Self(perm)
    }
    pub fn unit() -> Self {
        Self(Permutation::unit())
    }
    pub fn compose(&self, other: &Self) -> Self {
        Self(self.0.compose(&other.0))
    }
    pub fn inverse(&self) -> Self {
        Self(self.0.inverse())
    }
}

impl<const N: usize> std::ops::Add for &Alternating<N> {
    type Output = Alternating<N>;

    fn add(self, rhs: Self) -> Self::Output {
        self.compose(rhs)
    }
}

impl<const N: usize> std::ops::Neg for &Alternating<N> {
    type Output = Alternating<N>;

    fn neg(self) -> Self::Output {
        self.inverse()
    }
}

impl<const N: usize> std::ops::Sub for &Alternating<N> {
    type Output = Alternating<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.compose(&rhs.inverse())
    }
}

impl<const N: usize> std::ops::AddAssign<&Self> for Alternating<N> {
    fn add_assign(&mut self, rhs: &Self) {
        *self = self.compose(rhs)
    }
}

impl<const N: usize> std::ops::SubAssign<&Self> for Alternating<N> {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = self.compose(&rhs.inverse())
    }
}

impl<const N: usize> std::ops::Mul<usize> for &Alternating<N> {
    type Output = Alternating<N>;

    fn mul(self, rhs: usize) -> Self::Output {
        unsafe { Alternating::<N>::new_unchecked(self.0.mul(rhs)) }
    }
}

impl<const N: usize> Group for Alternating<N> {
    fn unit() -> Self {
        Self::unit()
    }
    fn inverse(&self) -> Self {
        self.inverse()
    }
    fn op(&self, rhs: Self) -> Self {
        self.compose(&rhs)
    }
}  
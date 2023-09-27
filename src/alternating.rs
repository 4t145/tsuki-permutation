use crate::{Commutator, Group, Permutation};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Alternating<const N: usize>(pub(crate) Permutation<N>);

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
    pub fn inverse(self) -> Self {
        Self(self.0.inverse())
    }
}

impl<const N: usize> Group for Alternating<N> {
    fn unit() -> Self {
        Self::unit()
    }
    fn inverse(self) -> Self {
        self.inverse()
    }
    fn op(self, rhs: &Self) -> Self {
        self.compose(rhs)
    }
}

impl<const N: usize> From<Commutator<Permutation<N>>> for Alternating<N> {
    fn from(comm: Commutator<Permutation<N>>) -> Self {
        Self(comm.eval())
    }
}

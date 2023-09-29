use crate::{Cyclic, Group};
#[derive(Debug, Clone)]
pub struct Dihedral<const N: usize> {
    // tau
    reflection: bool,
    // sigma
    rotation: Cyclic<N>,
}

impl<const N: usize> Dihedral<N> {
    const fn unit() -> Self {
        Self {
            reflection: false,
            rotation: Cyclic::unit(),
        }
    }
    const fn inverse(self) -> Self {
        Self {
            reflection: !self.reflection,
            rotation: self.rotation.inverse(),
        }
    }
    const fn op(self, rhs: &Self) -> Self {
        Self {
            reflection: self.reflection ^ rhs.reflection,
            rotation: self.rotation.compose(&rhs.rotation),
        }
    }
}

impl<const N: usize> Group for Dihedral<N> {
    fn unit() -> Self {
        Dihedral::unit()
    }
    fn inverse(self) -> Self {
        self.inverse()
    }
    fn op(self, rhs: &Self) -> Self {
        self.op(rhs)
    }
}

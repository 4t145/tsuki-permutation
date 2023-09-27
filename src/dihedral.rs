use crate::{Cyclic, Group};
#[derive(Debug, Clone)]
pub struct Dihedral<const N: usize> {
    // tau
    reflection: bool,
    // sigma
    rotation: Cyclic<N>,
}

impl<const N: usize> Group for Dihedral<N> {
    fn unit() -> Self {
        Self {
            reflection: false,
            rotation: Cyclic::unit(),
        }
    }
    fn inverse(self) -> Self {
        Self {
            reflection: !self.reflection,
            rotation: self.rotation.inverse(),
        }
    }
    fn op(self, rhs: &Self) -> Self {
        Self {
            reflection: self.reflection ^ rhs.reflection,
            rotation: self.rotation.compose(&rhs.rotation),
        }
    }
}

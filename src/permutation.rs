use std::fmt::Debug;

use crate::{Alternating, FiniteGroup, Group, Parity};
pub type S<const N: usize> = Permutation<N>;
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Permutation<const N: usize> {
    pub perm: [u8; N],
}

impl<const N: usize> Debug for Permutation<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.perm.iter()).finish()
    }
}

impl<const N: usize> Default for Permutation<N> {
    fn default() -> Self {
        Self::unit()
    }
}

impl<const N: usize> Permutation<N> {
    pub fn new(perm: [u8; N]) -> Self {
        for n in 0..N {
            assert!(perm.contains(&(n as u8)));
        }
        unsafe { Self::new_unchecked(perm) }
    }

    /// # Safety
    /// `perm` must be a permutation of `0..N`.
    pub unsafe fn new_unchecked(perm: [u8; N]) -> Self {
        Self { perm }
    }

    pub const fn unit() -> Self {
        let mut perm = [0; N];
        let mut idx = 0;
        while idx < N {
            perm[idx] = idx as u8;
            idx += 1;
        }
        Self { perm }
    }

    // O(n)
    pub const fn compose(&self, other: &Self) -> Self {
        let mut perm = [0; N];
        let mut idx = 0;
        while idx < N {
            perm[idx] = self.perm[other.perm[idx] as usize];
            idx += 1;
        }
        Self { perm }
    }

    // O(n)
    pub const fn inverse(&self) -> Self {
        let mut perm = [0; N];
        let mut idx = 0;
        while idx < N {
            perm[self.perm[idx] as usize] = idx as u8;
            idx += 1;
        }
        Self { perm }
    }

    // O(n)
    pub fn parity(&self) -> Parity {
        let mut perm = self.perm;
        let mut parity = Parity::Even;
        for i in 0..N {
            if perm[i] == i as u8 {
                continue;
            }
            let j = perm[i] as usize;
            perm.swap(i, j);
            parity = parity.flip();
        }
        parity
    }
}

impl<const N: usize> std::ops::Add for &Permutation<N> {
    type Output = Permutation<N>;

    fn add(self, rhs: Self) -> Self::Output {
        self.compose(rhs)
    }
}

impl<const N: usize> std::ops::Neg for &Permutation<N> {
    type Output = Permutation<N>;

    fn neg(self) -> Self::Output {
        self.inverse()
    }
}

impl<const N: usize> std::ops::Sub for &Permutation<N> {
    type Output = Permutation<N>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.compose(&rhs.inverse())
    }
}

impl<const N: usize> std::ops::AddAssign<&Self> for Permutation<N> {
    fn add_assign(&mut self, rhs: &Self) {
        *self = self.compose(rhs)
    }
}

impl<const N: usize> std::ops::SubAssign<&Self> for Permutation<N> {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = self.compose(&rhs.inverse())
    }
}

impl<const N: usize> std::ops::Mul<usize> for &Permutation<N> {
    type Output = Permutation<N>;

    fn mul(self, rhs: usize) -> Self::Output {
        let mut p = Permutation::<N>::unit();
        for _ in 0..rhs {
            p = p.compose(self);
        }
        p
    }
}

impl<const N: usize> Group for Permutation<N> {
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

impl<const N: usize> FiniteGroup for Permutation<N> {
    fn order() -> usize {
        N
    }

    fn enumerate() -> Box<dyn Iterator<Item = Self>>
    where
        Self: Sized,
    {
        Box::new((0..N).map(|i| {
            let mut perm = [0; N];
            let mut idx = 0;
            while idx < N {
                perm[idx] = (idx + i) as u8 % N as u8;
                idx += 1;
            }
            unsafe { Self::new_unchecked(perm) }
        }))
    }
}

impl<const N: usize> From<Alternating<N>> for Permutation<N> {
    fn from(val: Alternating<N>) -> Self {
        val.0
    }
}

#[macro_export]
macro_rules! perm {
    [] => {
        $crate::Permutation::<0>::unit()
    };
    [$($units:expr),+$(,)?] => {
        {
            let perm = [$($units),+];
            $crate::Permutation::new(perm)
        }
    };
    [@ $N:literal] => {
        {
            $crate::Permutation::<$N>::unit()
        }
    };
}

#[test]
fn test_permutaion() {
    let p = perm![1, 2, 0];
    let q = perm![2, 1, 0];
    let e3 = perm![@3];
    assert_eq!(&e3 + &p, p);
    assert_eq!(&p + &e3, p);
    assert_eq!(&p * 3, e3);
    assert_eq!(&p * 2, &e3 - &p);
    dbg!(p);
    assert_eq!(p.parity(), Parity::Even);
    assert_eq!((&p + &p).parity(), Parity::Even);
    assert_eq!((&q + &q).parity(), Parity::Even);
    assert_eq!((&q + &p).parity(), Parity::Odd);
    assert_eq!((&p + &q).parity(), Parity::Odd);
}

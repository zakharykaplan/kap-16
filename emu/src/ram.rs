use std::fmt::{self, Debug, Display};
use std::mem;
use std::ops::{Deref, DerefMut, Index, IndexMut};

use super::{uarch, WORDSIZE};

#[derive(Debug)]
pub struct Ram<const N: usize>(pub [u8; N]);

impl<const N: usize> Display for Ram<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const ROWSIZE: usize = mem::size_of::<usize>();
        for (i, row) in self.chunks(ROWSIZE).enumerate() {
            if row.iter().all(|&word| word == 0) {
                continue;
            }
            if i != 0 {
                writeln!(f)?;
            }
            write!(f, "{:#06x}:", WORDSIZE * ROWSIZE * i)?;
            for word in row {
                write!(f, " {:04x}", word)?;
            }
        }
        write!(f, "")
    }
}

impl<const N: usize> Default for Ram<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> Deref for Ram<N> {
    type Target = [uarch];

    fn deref(&self) -> &Self::Target {
        unsafe { self.0.align_to::<uarch>().1 }
    }
}

impl<const N: usize> DerefMut for Ram<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.0.align_to_mut::<uarch>().1 }
    }
}

impl<const N: usize> Index<usize> for Ram<N> {
    type Output = uarch;

    fn index(&self, idx: usize) -> &Self::Output {
        assert!((idx % 2) == 0);
        unsafe { &self.0.align_to::<uarch>().1[idx / WORDSIZE] }
    }
}

impl<const N: usize> IndexMut<usize> for Ram<N> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        assert!((idx % 2) == 0);
        unsafe { &mut self.0.align_to_mut::<uarch>().1[idx / WORDSIZE] }
    }
}

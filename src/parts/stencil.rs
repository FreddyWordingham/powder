//! World indexing.

use arctk::{
    clone,
    ord::{X, Y},
};

/// World index.
pub struct Stencil {
    /// Current index.
    index: [usize; 2],
}

impl Stencil {
    clone!(index, [usize; 2]);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(index: [usize; 2]) -> Self {
        Self { index }
    }

    /// Get the index below.
    #[inline]
    #[must_use]
    pub const fn under(&self) -> [usize; 2] {
        [self.index[X], self.index[Y] - 1]
    }

    /// Get the index below and to the left.
    #[inline]
    #[must_use]
    pub const fn under_left(&self) -> [usize; 2] {
        [self.index[X] - 1, self.index[Y] - 1]
    }

    /// Get the index below and to the right.
    #[inline]
    #[must_use]
    pub const fn under_right(&self) -> [usize; 2] {
        [self.index[X] + 1, self.index[Y] - 1]
    }
}

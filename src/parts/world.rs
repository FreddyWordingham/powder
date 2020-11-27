//! Simulation data.

use crate::parts::Spec;
use arctk::ord::{X, Y};
use ndarray::Array2;

/// Simulation data.
pub struct World {
    /// Cell data.
    cells: Array2<Spec>,
}

impl World {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            cells: Array2::default(res),
        }
    }
}

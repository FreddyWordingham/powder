//! Simulation data.

use crate::parts::{palette::*, Spec};
use arctk::ord::{X, Y};
use ndarray::Array2;
use rand::rngs::ThreadRng;

/// Simulation data.
pub struct World {
    /// Resolution.
    res: [usize; 2],
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

        let mut cells = Array2::default(res);
        for xi in 0..res[X] {
            cells[[xi, 0]] = Spec::Wall;
            cells[[xi, res[Y] - 1]] = Spec::Wall;
        }
        for yi in 0..res[Y] {
            cells[[0, yi]] = Spec::Wall;
            cells[[res[X] - 1, yi]] = Spec::Wall;
        }

        Self { res, cells }
    }

    /// Tick forward one instance.
    #[inline]
    pub fn tick(&mut self, mut _rng: &mut ThreadRng) {
        for xi in 2..(self.res[X] - 2) {
            for yi in 2..(self.res[Y] - 2) {
                self.cells[[xi, yi]].evolve();
            }
        }
    }

    /// Draw the world state to a buffer.
    #[inline]
    pub fn draw(&self, buffer: &mut Vec<u32>) {
        let length = buffer.len();

        for yi in 0..self.res[Y] {
            let offset = yi * self.res[X];
            for xi in 0..self.res[X] {
                let index = offset + self.res[X] - xi;
                buffer[length - index] = match self.cells[[xi, yi]] {
                    Spec::Empty => EMPTY,
                    Spec::Wall => WALL,
                }
            }
        }
    }
}

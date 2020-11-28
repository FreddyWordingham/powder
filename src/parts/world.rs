//! Simulation data.

use crate::parts::{palette::*, Spec};
use arctk::ord::{X, Y};
use ndarray::Array2;
use std::mem;

/// Simulation data.
pub struct World {
    /// Resolution.
    res: [usize; 2],
    /// Cell data.
    cells: Array2<Spec>,
    /// Buffer data.
    buffer: Array2<Spec>,
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

        Self {
            res,
            cells,
            buffer: Array2::default(res),
        }
    }

    /// Tick forward one instance.
    #[inline]
    pub fn tick(&mut self) {
        self.cells[[50, 100]] = Spec::Sand;

        for yi in 1..(self.res[Y] - 1) {
            for xi in 1..(self.res[X] - 1) {
                match self.cells[[xi, yi]] {
                    Spec::Sand => {
                        if self.buffer[[xi, yi - 1]] == Spec::Empty {
                            self.cells[[xi, yi]] = Spec::Empty;
                            self.buffer[[xi, yi - 1]] = Spec::Sand;
                        } else if self.buffer[[xi - 1, yi - 1]] == Spec::Empty {
                            self.cells[[xi, yi]] = Spec::Empty;
                            self.buffer[[xi - 1, yi - 1]] = Spec::Sand;
                        } else if self.buffer[[xi + 1, yi - 1]] == Spec::Empty {
                            self.cells[[xi, yi]] = Spec::Empty;
                            self.buffer[[xi + 1, yi - 1]] = Spec::Sand;
                        } else {
                            self.cells[[xi, yi]] = Spec::Empty;
                            self.buffer[[xi, yi]] = Spec::Sand;
                        }
                    }
                    _ => {}
                }
            }
        }

        mem::swap(&mut self.cells, &mut self.buffer);
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
                    Spec::Wall => WALL,
                    Spec::Empty => EMPTY,
                    Spec::Sand => SAND,
                }
            }
        }
    }
}

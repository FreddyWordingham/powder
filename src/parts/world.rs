//! Simulation data.

use crate::parts::{palette::*, Spec, Stencil};
use arctk::ord::{X, Y};
use ndarray::Array2;
use rand::rngs::ThreadRng;
use std::mem;

/// Simulation data.
pub struct World {
    /// Resolution.
    res: [usize; 2],
    /// Cell data.
    cells: Array2<Spec>,
    /// Buffer data.
    buffer: Array2<Spec>,
    /// Sources.
    sources: Array2<Spec>,
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
            sources: Array2::default(res),
        }
    }

    /// Create a species source.
    #[inline]
    pub fn set_source(&mut self, spec: Spec, index: [usize; 2]) {
        debug_assert!(index[X] < self.res[X]);
        debug_assert!(index[Y] < self.res[Y]);

        self.sources[index] = spec;
    }

    /// Tick forward one instance.
    #[inline]
    pub fn tick(&mut self, rng: &mut ThreadRng) {
        let curr = &mut self.cells;
        let next = &mut self.buffer;

        // next[[50, 100]] = Spec::Sand;
        // next[[101, 100]] = Spec::Water;

        // next += self.sources;

        for yi in 0..self.res[Y] {
            for xi in 0..self.res[X] {
                let index = Stencil::new([xi, yi]);

                match curr[index.index()] {
                    Spec::Wall => {
                        curr[index.index()] = Spec::Empty;
                        next[index.index()] = Spec::Wall;
                    }
                    Spec::Sand => {
                        if next[index.under()] == Spec::Empty {
                            curr[index.index()] = Spec::Empty;
                            next[index.under()] = Spec::Sand;
                        } else if next[index.under_left()] == Spec::Empty {
                            curr[index.index()] = Spec::Empty;
                            curr[index.under_left()] = Spec::Sand;
                        } else if next[index.under_right()] == Spec::Empty {
                            curr[index.index()] = Spec::Empty;
                            curr[index.under_right()] = Spec::Sand;
                        } else {
                            curr[index.index()] = Spec::Empty;
                            next[index.index()] = Spec::Sand;
                        }
                    }
                    Spec::Water => {
                        if next[index.under()] == Spec::Empty {
                            curr[index.index()] = Spec::Empty;
                            next[index.under()] = Spec::Water;
                        } else if next[index.under_left()] == Spec::Empty {
                            curr[index.index()] = Spec::Empty;
                            curr[index.under_left()] = Spec::Water;
                        } else if next[index.under_right()] == Spec::Empty {
                            curr[index.index()] = Spec::Empty;
                            curr[index.under_right()] = Spec::Water;
                        } else if next[index.left()] == Spec::Empty {
                            curr[index.index()] = Spec::Empty;
                            next[index.left()] = Spec::Water;
                        } else if next[index.right()] == Spec::Empty {
                            curr[index.index()] = Spec::Empty;
                            next[index.right()] = Spec::Water;
                        } else {
                            curr[index.index()] = Spec::Empty;
                            next[index.index()] = Spec::Sand;
                        }
                    }
                    _ => {}
                }
            }
        }

        mem::swap(curr, next);
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
                    Spec::Water => WATER,
                }
            }
        }
    }
}

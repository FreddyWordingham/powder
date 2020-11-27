//! Simulation data.

use crate::parts::Spec;
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
        for xi in 54..74 {
            for yi in 54..74 {
                cells[[xi, yi]] = Spec::Sand;
            }
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
        for yi in 0..self.res[Y] {
            if yi > 0 {
                for xi in 0..self.res[X] {
                    match self.cells[[xi, yi]] {
                        Spec::Empty => {}
                        Spec::Sand => {
                            if self.buffer[[xi, yi - 1]] == Spec::Empty {
                                self.buffer[[xi, yi - 1]] = Spec::Sand;
                            }
                        }
                    }
                }
            }
        }

        mem::swap(&mut self.cells, &mut self.buffer);
    }

    /// Draw the world state to a buffer.
    #[inline]
    pub fn draw(&self, buffer: &mut Vec<u32>) {
        for (p, s) in buffer.iter_mut().zip(&self.cells) {
            match s {
                Spec::Sand => {
                    *p = components_to_u32(100, 0, 0);
                }
                _ => {}
            }
        }
    }
}

/// Create a 32 bit colour representation from 8 bit components.
#[inline]
#[must_use]
pub const fn components_to_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}
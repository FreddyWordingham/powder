//! Simulation data.

use crate::parts::{order::*, palette::*, Spec};
use arctk::ord::{X, Y};
use ndarray::Array2;
use rand::{rngs::ThreadRng, Rng};

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

        cells[[47, 50]] = Spec::Wall;
        cells[[48, 50]] = Spec::Wall;
        cells[[49, 50]] = Spec::Wall;
        cells[[50, 50]] = Spec::Wall;
        cells[[51, 50]] = Spec::Wall;
        cells[[52, 50]] = Spec::Wall;
        cells[[53, 50]] = Spec::Wall;

        Self { res, cells }
    }

    /// Tick forward one instance.
    #[inline]
    pub fn tick(&mut self, mut rng: &mut ThreadRng) {
        self.cells[[50, 70]] = Spec::Sand;
        self.cells[[150, 70]] = Spec::Water;

        for yi in 0..self.res[Y] {
            let y = yi as i32;
            for xi in 0..self.res[X] {
                let x = xi as i32;
                let index = [xi, yi];

                match self.cells[index] {
                    Spec::Wall => {}
                    Spec::Empty => {}
                    Spec::Sand => {
                        self.sand_reaction(&mut rng, x, y);
                    }
                    Spec::Water => {
                        self.water_reaction(&mut rng, x, y);
                    }
                }
            }
        }
    }

    /// Attempt to react a sand spec.
    #[inline]
    fn sand_reaction(&mut self, rng: &mut ThreadRng, x: i32, y: i32) {
        let cells = &mut self.cells;

        let index = [x as usize, y as usize];
        for [dx, dy] in &POWDER[rng.gen_range(0, 2)] {
            let other_index = [(x + dx) as usize, (y + dy) as usize];
            let mut other = cells[other_index];

            if let Some(product) = other.react(cells[index]) {
                cells[index] = product;
                cells[other_index] = other;
                return;
            }
        }
    }

    /// Attempt to react a water spec.
    #[inline]
    fn water_reaction(&mut self, rng: &mut ThreadRng, x: i32, y: i32) {
        let cells = &mut self.cells;

        let index = [x as usize, y as usize];
        for [dx, dy] in &LIQUID[rng.gen_range(0, 2)] {
            let other_index = [(x + dx) as usize, (y + dy) as usize];
            let mut other = cells[other_index];

            if let Some(product) = other.react(cells[index]) {
                cells[index] = product;
                cells[other_index] = other;
                return;
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
                    Spec::Wall => WALL,
                    Spec::Empty => EMPTY,
                    Spec::Sand => SAND,
                    Spec::Water => WATER,
                }
            }
        }
    }
}

//! Simulation data.

use crate::parts::{order::*, palette::*, Spec};
use arctk::ord::{X, Y};
use ndarray::Array2;
use rand::{prelude::SliceRandom, rngs::ThreadRng, Rng};

/// Simulation data.
pub struct World {
    /// Resolution.
    res: [usize; 2],
    /// Cell data.
    cells: Array2<Spec>,
    /// Update order.
    order: Vec<[i32; 2]>,
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

        let mut order = Vec::with_capacity(res[X] * res[Y]);
        for xi in 0..res[X] {
            let x = xi as i32;
            for yi in 0..res[Y] {
                let y = yi as i32;

                order.push([x, y]);
            }
        }

        Self { res, cells, order }
    }

    /// Tick forward one instance.
    #[inline]
    pub fn tick(&mut self, mut rng: &mut ThreadRng) {
        self.cells[[50, 70]] = Spec::Sand;
        self.cells[[100, 70]] = Spec::Water;
        self.cells[[150, 70]] = Spec::Oil;
        self.cells[[200, 70]] = Spec::Fire(4);

        let mut order = self.order.clone();
        order.shuffle(&mut rng);
        for [x, y] in order {
            let index = [x as usize, y as usize];

            self.cells[index].tick();
            match self.cells[index] {
                Spec::Wall => {}
                Spec::Empty => {}
                Spec::Sand => {
                    self.sand_reaction(&mut rng, x, y);
                }
                Spec::Water => {
                    self.water_reaction(&mut rng, x, y);
                }
                Spec::Oil => {
                    self.oil_reaction(&mut rng, x, y);
                }
                Spec::Fire(..) => {
                    self.fire_reaction(&mut rng, x, y);
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

    /// Attempt to react an oil spec.
    #[inline]
    fn oil_reaction(&mut self, rng: &mut ThreadRng, x: i32, y: i32) {
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

    /// Attempt to react a fire spec.
    #[inline]
    fn fire_reaction(&mut self, rng: &mut ThreadRng, x: i32, y: i32) {
        let cells = &mut self.cells;

        let index = [x as usize, y as usize];
        for [dx, dy] in &GAS[rng.gen_range(0, 8)] {
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
                    Spec::Oil => OIL,
                    Spec::Fire(..) => FIRE,
                }
            }
        }
    }
}

//! Simulation data.

use crate::parts::{palette::*, spec::Spec, Particle};
use arctk::{
    math::{Pos2, Vec2},
    ord::{X, Y},
};
use ndarray::Array2;
use rand::rngs::ThreadRng;
use std::f64::consts::PI;

/// Simulation data.
pub struct World {
    /// Resolution.
    res: [usize; 2],
    /// Cell data.
    cells: Array2<Spec>,
    /// Particle data.
    parts: Vec<Particle>,
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

        let num_parts = 1000;
        let mut parts = Vec::with_capacity(num_parts);
        for n in 0..num_parts {
            let theta = n as f64 * ((2.0 * PI) / num_parts as f64);
            let rho = 1.0e-4;
            parts.push(Particle::new(
                Pos2::new(res[X] as f64 / 2.0, res[Y] as f64 / 2.0),
                Vec2::new(theta.sin() * rho * n as f64, theta.cos() * rho * n as f64),
            ));
        }

        Self { res, cells, parts }
    }

    /// Tick forward one instance.
    #[inline]
    pub fn tick(&mut self, mut _rng: &mut ThreadRng) {
        // Cells.
        for xi in 2..(self.res[X] - 2) {
            for yi in 2..(self.res[Y] - 2) {
                let neighbours = [
                    self.cells[[xi - 1, yi]],
                    self.cells[[xi + 1, yi]],
                    self.cells[[xi, yi - 1]],
                    self.cells[[xi, yi + 1]],
                ];
                self.cells[[xi, yi]] = self.cells[[xi, yi]].evolve(&neighbours);
            }
        }

        // Particles.
        for part in &mut self.parts {
            let xi = part.pos.x as usize;
            let yi = part.pos.y as usize;
            let neighbours = [
                self.cells[[xi - 1, yi]],
                self.cells[[xi + 1, yi]],
                self.cells[[xi, yi - 1]],
                self.cells[[xi, yi + 1]],
            ];
            part.evolve(&neighbours);
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
                    Spec::Firework(n) => {
                        println!("fire {}", n);
                        FIRE
                    }
                }
            }
        }

        for part in &self.parts {
            let offset = part.pos.y as usize * self.res[X];
            let index = offset + self.res[X] - part.pos.x as usize;
            buffer[length - index] = WATER;
        }
    }
}

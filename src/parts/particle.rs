//! Floating particle instance.

use crate::parts::Spec;
use arctk::math::{Pos2, Vec2};

/// Particle structure.
pub struct Particle {
    /// Position.
    pub pos: Pos2,
    /// Velocity.
    pub vel: Vec2,
}

impl Particle {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(pos: Pos2, vel: Vec2) -> Self {
        Self { pos, vel }
    }

    /// Evolve the particle forward a single tick.
    #[inline]
    pub fn evolve(&mut self, neighbours: &[Spec; 4]) {
        if neighbours[0] == Spec::Wall || neighbours[1] == Spec::Wall {
            self.vel.x *= -1.0;
        }
        if neighbours[2] == Spec::Wall || neighbours[3] == Spec::Wall {
            self.vel.y *= -1.0;
        }

        self.pos += self.vel;
    }
}

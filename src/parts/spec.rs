//! Enumeration of known species.

/// Species kinds
#[derive(PartialEq, Copy, Clone)]
pub enum Spec {
    /// Empty.
    Empty,
    /// Wall.
    Wall,
    /// Firework (lifetime).
    Firework(i32),
}

impl Spec {
    /// Evolve the particle forward a single tick.
    #[must_use]
    #[inline]
    pub const fn evolve(self, _neighbours: &[Self; 4]) -> Self {
        match self {
            Self::Firework(fuel) => {
                if fuel > 0 {
                    Self::Firework(fuel - 1)
                } else {
                    Self::Empty
                }
            }
            Self::Wall => Self::Wall,
            Self::Empty => Self::Empty,
        }
    }
}

impl Default for Spec {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::Empty
    }
}

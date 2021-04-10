//! Enumeration of known species.

/// Species kinds
#[derive(PartialEq, Copy, Clone)]
pub enum Spec {
    /// Empty.
    Empty,
    /// Wall.
    Wall,
}

impl Spec {
    /// Evolve the particle forward a single tick.
    #[inline]
    pub fn evolve(&mut self) {
        match self {
            _ => {}
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

//! Enumeration of known species.

/// Species kinds
#[derive(PartialEq, Copy, Clone)]
pub enum Spec {
    /// Wall.
    Wall,
    /// Empty.
    Empty,
    /// Sand.
    Sand,
    /// Water.
    Water,
}

impl Default for Spec {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::Empty
    }
}

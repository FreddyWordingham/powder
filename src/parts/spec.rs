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

impl Spec {
    /// Attempt a reaction between this spec and an incoming spec.
    /// Return None if no reaction will occur.
    /// Return Some<Self> if the reaction proceeds.
    /// The wrapped returned value is a product of the reaction.
    #[inline]
    #[must_use]
    pub fn react(&mut self, rhs: Self) -> Option<Self> {
        match self {
            Self::Wall => {
                return None;
            }
            Self::Empty => {
                *self = rhs;
                return Some(Self::Empty);
            }
            Self::Sand => {
                return None;
            }
            Self::Water => {
                return None;
            }
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

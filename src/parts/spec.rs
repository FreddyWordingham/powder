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
    /// Oil.
    Oil,
    /// Fire.
    Fire(i32),
}

impl Spec {
    /// Evolve the particle forward a single tick.
    #[inline]
    pub fn tick(&mut self) {
        match *self {
            Self::Fire(t) => {
                if t > 0 {
                    *self = Self::Fire(t - 1);
                } else {
                    *self = Self::Empty;
                }
            }
            _ => {}
        }
    }

    /// Attempt a reaction between this spec and an incoming spec.
    /// Return None if no reaction will occur.
    /// Return Some<Self> if the reaction proceeds.
    /// The wrapped returned value is a product of the reaction.
    #[inline]
    #[must_use]
    pub fn react(&mut self, rhs: Self) -> Option<Self> {
        match *self {
            Self::Wall => None,
            Self::Empty => {
                *self = rhs;
                Some(Self::Empty)
            }
            Self::Sand => None,
            Self::Water => match rhs {
                Self::Sand => {
                    *self = rhs;
                    Some(Self::Water)
                }
                _ => None,
            },
            Self::Oil => match rhs {
                Self::Sand | Self::Water => {
                    *self = rhs;
                    Some(Self::Oil)
                }
                _ => None,
            },
            Self::Fire(time) => match rhs {
                Self::Sand | Self::Water => {
                    *self = rhs;
                    Some(Self::Empty)
                }
                Self::Oil => {
                    *self = Self::Fire(4);
                    Some(Self::Fire(time))
                }
                _ => None,
            },
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

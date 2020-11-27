//! Enumeration of known species.

/// Species kinds
#[derive(PartialEq, Copy, Clone)]
pub enum Spec {
    /// Empty.
    Empty,
    /// Sand.
    Sand,
}

impl Default for Spec {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::Empty
    }
}

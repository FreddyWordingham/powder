//! Enumeration of known species.

use std::ops::{Add, AddAssign};

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

// impl Add for Spec {
//     type Output = Self;

//     #[inline]
//     #[must_use]
//     fn add(self, rhs: Self) -> Self::Output {
//         match self {
//             Self::Empty => {
//                 rhs
//             },
//             Self::Sand => {

//             }
//         }
//     }
// }

// impl AddAssign for Spec {
//     #[inline]
//     #[must_use]
//     fn add_assign(&mut self, rhs: Self) {
//         // if rhs == Self::Empty {
//         //     self
//         // } else {
//         //     rhs
//         // }

//         match self {

//         }
//     }
// }

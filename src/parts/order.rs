//! Reaction order.

use crate::parts::direction::{DOWN, DOWN_LEFT, DOWN_RIGHT, LEFT, RIGHT, UP, UP_LEFT, UP_RIGHT};

/// Solid object.
pub const SOLID: [[i32; 2]; 1] = [DOWN];

/// Powder object.
pub const POWDER: [[[i32; 2]; 3]; 2] =
    [[DOWN, DOWN_LEFT, DOWN_RIGHT], [DOWN, DOWN_RIGHT, DOWN_LEFT]];

/// Liquid object.
pub const LIQUID: [[[i32; 2]; 5]; 2] = [
    [DOWN, DOWN_LEFT, DOWN_RIGHT, LEFT, RIGHT],
    [DOWN, DOWN_RIGHT, DOWN_LEFT, RIGHT, LEFT],
];

/// Gas object.
pub const GAS: [[i32; 2]; 8] = [
    DOWN, DOWN_LEFT, DOWN_RIGHT, LEFT, RIGHT, UP, UP_LEFT, UP_RIGHT,
];

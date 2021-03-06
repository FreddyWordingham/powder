//! Colour constants.

/// Create a 32 bit colour representation from 8 bit components.
#[inline]
#[must_use]
pub const fn components_to_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Filled obstruction colour.
pub const WALL: u32 = components_to_u32(50, 50, 50);

/// Empty void colour.
pub const EMPTY: u32 = components_to_u32(10, 10, 10);

/// Alive colour.
pub const ALIVE: u32 = components_to_u32(255, 255, 255);

/// Sand colour.
pub const SAND: u32 = components_to_u32(255, 204, 0);

/// Water colour.
pub const WATER: u32 = components_to_u32(100, 204, 255);

/// Oil colour.
pub const OIL: u32 = components_to_u32(150, 75, 0);

/// Fire colour.
pub const FIRE: u32 = components_to_u32(255, 75, 25);

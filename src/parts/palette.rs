//! Colour constants.

/// Create a 32 bit colour representation from 8 bit components.
#[inline]
#[must_use]
pub const fn components_to_u32(r: u8, g: u8, b: u8) -> u32 {
    ((r as u32) << 16) | ((g as u32) << 8) | (b as u32)
}

/// Sand colour.
pub const SAND: u32 = components_to_u32(100, 0, 0);

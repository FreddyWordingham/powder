//! Cell size options.

use arctk_attr::input;
use minifb::Scale;

/// Cell scaling.
#[input]
pub enum CellSize {
    /// 1 pixel per cell.
    X1,
    /// 4 pixels per cell.
    X2,
    /// 16 pixels per cell.
    X4,
    /// 64 pixels per cell.
    X8,
    /// 256 pixels per cell.
    X16,
    /// 1024 pixels per cell.
    X32,
}

impl CellSize {
    /// Convert the type.
    #[inline]
    #[must_use]
    pub const fn scale(self) -> Scale {
        match self {
            Self::X1 => Scale::X1,
            Self::X2 => Scale::X2,
            Self::X4 => Scale::X4,
            Self::X8 => Scale::X8,
            Self::X16 => Scale::X16,
            Self::X32 => Scale::X32,
        }
    }
}

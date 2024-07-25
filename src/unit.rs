use std::fmt::Debug;
use std::ops::{Add, Deref, Div, Mul, Rem, Sub};

use serde::{Deserialize, Serialize};

/// A unit of measure.
pub trait Unit:
    Sized
    + Deref
    + Add<Output = Self>
    + Div<Output = Self>
    + Mul<Output = Self>
    + Rem<Output = Self>
    + Sub<Output = Self>
    + Debug
    + Default
    + Clone
    + Copy
    + PartialEq
    + PartialOrd
    + Serialize
    + for<'de> Deserialize<'de>
{
    /// Returns units of _points_.
    fn pt(&self) -> f32;
}

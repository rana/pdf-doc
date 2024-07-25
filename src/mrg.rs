use crate::inch::In;
use serde::{Deserialize, Serialize};

/// A margin with _1in_ for the left, right, bottom, and top.
pub const MRG_IN_1: Mrg = Mrg {
    lft: In(1.0),
    rht: In(1.0),
    btm: In(1.0),
    top: In(1.0),
};

/// A margin with a _left_, _right_, _bottom_, _top_.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Mrg {
    /// The _left_ margin.
    pub lft: In,
    /// The _right_ margin.
    pub rht: In,
    /// The _bottom_ margin.
    pub btm: In,
    /// The _top_ margin.
    pub top: In,
}

impl Mrg {
    /// Returns a new [`Mrg`].
    pub fn new(lft: In, rht: In, btm: In, top: In) -> Self {
        Self { lft, rht, btm, top }
    }

    pub fn width(&self) -> In {
        self.lft + self.rht
    }

    pub fn height(&self) -> In {
        self.btm + self.top
    }
}

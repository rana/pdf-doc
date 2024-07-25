use crate::{inch::In, unit::Unit};
use serde::{Deserialize, Serialize};

/// An _8.5in x 11in_ letter size.
///
/// ANSI (American National Standards Institute) letter size,
/// also known as ANSI A, is a standard paper size in the United States.
pub const ANSI_LETTER: Sze = Sze {
    width: In(8.5),
    height: In(11.0),
};

/// A size with a _width_ and _height_.
#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Sze {
    pub width: In,
    pub height: In,
}

impl Sze {
    /// Returns a new [`Sze`].
    pub fn new(width: In, height: In) -> Self {
        Self { width, height }
    }

    /// Returns a tuple in units of _points_.
    pub fn pt(&self) -> (f32, f32) {
        (self.width.pt(), self.height.pt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inch::In;
    use serde_json;

    #[test]
    fn test_round_trip_serialize_deserialize() {
        let original = Sze {
            width: In(8.5),
            height: In(11.0),
        };

        // Serialize the `Size` instance to a JSON string
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");
        // eprintln!("{serialized}");

        // Deserialize the JSON string back to a `Size` instance
        let deserialized: Sze = serde_json::from_str(&serialized).expect("Failed to deserialize");

        // Check that the original and deserialized instances are the same
        assert_eq!(original, deserialized);
    }
}

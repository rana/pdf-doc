use crate::unit::*;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Deref, Div, Mul, Rem, Sub},
};

/// Points per inch.
pub const PT_PER_IN: f32 = 72.0;

/// A length in inches.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct In(pub f32);

impl Display for In {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}in", self.0)
    }
}

impl Deref for In {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Unit for In {
    fn pt(&self) -> f32 {
        self.0 * PT_PER_IN
    }
}

// Implement arithmetic operations with two operands of `In`

impl Add for In {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        In(self.0 + rhs.0)
    }
}

impl Sub for In {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        In(self.0 - rhs.0)
    }
}

impl Mul for In {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        In(self.0 * rhs.0)
    }
}

impl Div for In {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        In(self.0 / rhs.0)
    }
}

impl Rem for In {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        In(self.0 % rhs.0)
    }
}

// Implement arithmetic operations with all primitive numeric types
macro_rules! impl_numeric_ops {
    ($($t:ty)*) => {
        $(
            impl Add<$t> for In {
                type Output = Self;

                fn add(self, rhs: $t) -> Self {
                    In(self.0 + rhs as f32)
                }
            }

            impl Sub<$t> for In {
                type Output = Self;

                fn sub(self, rhs: $t) -> Self {
                    In(self.0 - rhs as f32)
                }
            }

            impl Mul<$t> for In {
                type Output = Self;

                fn mul(self, rhs: $t) -> Self {
                    In(self.0 * rhs as f32)
                }
            }

            impl Div<$t> for In {
                type Output = Self;

                fn div(self, rhs: $t) -> Self {
                    In(self.0 / rhs as f32)
                }
            }

            impl Rem<$t> for In {
                type Output = Self;

                fn rem(self, rhs: $t) -> Self {
                    In(self.0 % rhs as f32)
                }
            }
        )*
    }
}

impl_numeric_ops!(i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let length = In(12.34);
        let expected_output = "12.34in";
        let actual_output = format!("{}", length);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_to_pt() {
        let length = In(1.0); // 1 inch
        let expected_points = 72.0; // 1 inch = 72 points
        let actual_points = length.pt();
        assert_eq!(actual_points, expected_points);
    }

    #[test]
    fn test_round_trip_serialize_deserialize() {
        let original = In(12.34);

        // Serialize the `In` instance to a JSON string
        let serialized = serde_json::to_string(&original).expect("Failed to serialize");

        // Deserialize the JSON string back to an `In` instance
        let deserialized: In = serde_json::from_str(&serialized).expect("Failed to deserialize");

        // Check that the original and deserialized instances are the same
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_add_in() {
        let a = In(5.0);
        let b = In(3.0);
        let result = a + b;
        assert_eq!(result.0, 8.0);
    }

    #[test]
    fn test_sub_in() {
        let a = In(5.0);
        let b = In(3.0);
        let result = a - b;
        assert_eq!(result.0, 2.0);
    }

    #[test]
    fn test_mul_in() {
        let a = In(5.0);
        let b = In(3.0);
        let result = a * b;
        assert_eq!(result.0, 15.0);
    }

    #[test]
    fn test_div_in() {
        let a = In(10.0);
        let b = In(2.0);
        let result = a / b;
        assert_eq!(result.0, 5.0);
    }

    #[test]
    fn test_rem_in() {
        let a = In(10.0);
        let b = In(3.0);
        let result = a % b;
        assert_eq!(result.0, 1.0);
    }

    macro_rules! test_numeric_ops {
        ($($name:ident: $t:ty, $val:expr)*) => {
            $(
                #[test]
                fn $name() {
                    let a = In(5.0);
                    let result_add = a + $val as $t;
                    let result_sub = a - $val as $t;
                    let result_mul = a * $val as $t;
                    let result_div = a / $val as $t;
                    let result_rem = a % $val as $t;

                    assert_eq!(result_add.0, 5.0 + $val as f32);
                    assert_eq!(result_sub.0, 5.0 - $val as f32);
                    assert_eq!(result_mul.0, 5.0 * $val as f32);
                    assert_eq!(result_div.0, 5.0 / $val as f32);
                    assert_eq!(result_rem.0, 5.0 % $val as f32);
                }
            )*
        }
    }

    test_numeric_ops! {
        test_add_i8: i8, 2
        test_add_i16: i16, 2
        test_add_i32: i32, 2
        test_add_i64: i64, 2
        test_add_i128: i128, 2
        test_add_u8: u8, 2
        test_add_u16: u16, 2
        test_add_u32: u32, 2
        test_add_u64: u64, 2
        test_add_u128: u128, 2
        test_add_f32: f32, 2.0
        test_add_f64: f64, 2.0
    }
}

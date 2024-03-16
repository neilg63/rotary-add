use std::ops::{Add, Sub};

/// Trait with methods for cyclical or rotary arithmetic with unsigned integers
/// When additions or subtractions may otherwise overflow, 
/// either falling below zero or above the type's maximum,
/// the values rotate. This is mainly for use with cryptography
pub trait RotaryAdd<T: Add<Output = T> + Sub<Output = T> + PartialEq + Copy> {
  /// Add another unsigned integer and start from zero again should value overflow,
  /// but retaining the remainder 
  /// With u8, 255.rotary_add(4) is thus 3
  fn rotary_add(&self, other: &T) -> T;

  /// Subtract another unsigned integer and start from the type's max value 
  /// if the target result may be otherwise be negative if it were a signed integer
  /// e.g. with u8 the max is 255 (allowing for 256 values including zero)
  /// 3.rotary_sub(4) is thus 255
  fn rotary_sub(&self, other: &T) -> T;
}
/// Macro to implement the above for u8, u16 and u32
macro_rules! impl_rotary_add {
    ($t:ty,$u:ty, $max:expr) => {
        impl RotaryAdd<$t> for $t {
            fn rotary_add(&self, other: &$t) -> $t {
                let diff = $max - *other;
                if *self < diff {
                    *self + *other
                } else {
                  let radix = $max as $u + 1;
                  let result = *self as $u + *other as $u;
                  (result % radix) as $t
                }
            }

            fn rotary_sub(&self, other: &$t) -> $t {
                if *self < *other {
                  let radix = $max as $u + 1;
                  let result = *other as $u - *self as $u;
                  (radix - result) as $t
                } else {
                    *self - *other
                }
            }
        }
    };
}

impl_rotary_add!(u8, u16, u8::MAX);
impl_rotary_add!(u16, u32, u16::MAX);
impl_rotary_add!(u32, u64, u32::MAX);

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rotary_add_u8() {
    let num_1: u8 = 254;
    let num_2: u8 = 37;
    let result = num_1.rotary_add(&num_2);
    let expected_result = 35;
    assert_eq!(result, expected_result);
    let result_2: u8 = 3.rotary_sub(&4);
    let expected_result_2 = 255;
    assert_eq!(result_2, expected_result_2);
  }

  #[test]
  fn test_rotary_add_u32() {
    let num_1: u32 = 3998373629;
    let num_2: u32 = 998373673;
    let result = num_1.rotary_add(&num_2);
    let expected_result = 701780006;
    assert_eq!(result, expected_result);
  }

  #[test]
  fn test_rotary_add_u16() {
    let num_1: u16 = 64777;
    let num_2: u16 = 1000;
    let result = num_1.rotary_add(&num_2);
    let expected_result = 241;
    assert_eq!(result, expected_result);
  }

  #[test]
  fn test_rotary_sub_u8() {
    let num_1: u8 = 127;
    let num_2: u8 = 128;
    let result = num_1.rotary_sub(&num_2);
    let expected = 255;
    assert_eq!(result, expected);

    let num_1: u8 = 137;
    let num_2: u8 = 128;
    let result = num_1.rotary_sub(&num_2);
    let expected = 9;
    assert_eq!(result, expected);

    let num_1: u8 = 3;
    let num_2: u8 = 240;
    let result = num_1.rotary_sub(&num_2);
    let expected = 19;
    assert_eq!(result, expected);

    let num_1: u8 = 143;
    let num_2: u8 = 143;
    let result = num_1.rotary_sub(&num_2);
    let expected = 0;
    assert_eq!(result, expected);
  }

  #[test]
  fn test_rotary_sub_u32() {
    let num_1: u32 = 998373673;
    let num_2: u32 = 3998373629;
    let result = num_1.rotary_sub(&num_2);
    let expected = 1294967340;
    assert_eq!(result, expected);

    let num_1: u32 = 2998373673;
    let num_2: u32 = 2998373674;
    let result = num_1.rotary_sub(&num_2);
    let expected = 4294967295;
    assert_eq!(result, expected);
  }
}

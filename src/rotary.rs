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

// Implement for u8, max 255 (value range 256)
impl_rotary_add!(u8, u16, u8::MAX);

// Implement for u16, max 65535 (value range 65536)
impl_rotary_add!(u16, u32, u16::MAX);

// Implement for u32, max 4294967295 (value range 4294967296)
impl_rotary_add!(u32, u64, u32::MAX);
use std::ops::{Add, Sub};

/// Trait with methods for cyclical arithmetic with unsigned integers
/// When additions or subtractions may otherwise overflow, 
/// either falling below zero or above the type's maximum,
/// the values rotate. This is mainly for use with cryptography
pub trait CycleAdd<T: Add<Output = T> + Sub<Output = T> + PartialEq + Copy> {
  /// Add another unsigned integer and start from zero again should value overflow,
  /// but retaining the remainder 
  /// With u8, 255.cycle_add(4) is thus 3
  fn cycle_add(&self, other: &T, base: T) -> T;

  /// Subtract another unsigned integer and start from the type's max value 
  /// if the target result may be otherwise be negative if it were a signed integer
  /// e.g. with u8 the max is 255 (allowing for 256 values including zero)
  /// 3.cycle_sub(4) is thus 255
  fn cycle_sub(&self, other: &T, base: T) -> T;
}

/// Macro to implement the above for u8, u16 and u32
macro_rules! impl_cycle_add {
    ($t:ty,$u:ty) => {
        impl CycleAdd<$t> for $t {
            fn cycle_add(&self, other: &$t, base: $t) -> $t {
              let b2 = base as $u;
              let result: $u = *self as $u + *other as $u;
              if result < b2 {
                result as $t
              } else {
                (result % base as $u) as $t
              }
            }

            fn cycle_sub(&self, other: &$t, base: $t) -> $t {
                if *self < *other {
                  let b2 = base as $u;
                  let result = b2 + *self as $u - *other as $u ;
                  (result % b2) as $t
                } else {
                    *self - (*other % base)
                }
            }
        }
    };
}

// Implement for u8, max 255 (value range 256)
impl_cycle_add!(u8, u16);

// Implement for u16, max 65535 (value range 65536)
impl_cycle_add!(u16, u32);

// Implement for u32, max 4294967295 (value range 4294967296)
impl_cycle_add!(u32, u64);
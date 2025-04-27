use std::ops::{Add, Sub};

/// Trait with methods for cyclical arithmetic with unsigned integers
/// Values rotate within a range of zero to 1 below the specified base, 
/// when normal addition or subtraction would yield out-of-range values
pub trait CycleAdd<T: Add<Output = T> + Sub<Output = T> + PartialEq + Copy> {
  /// Add another unsigned integer and start from zero again if the result is out of range.
  /// With a a base of 60 applied to a u8 value, 53.cycle_add(10, 60) is thus 3
  fn cycle_add(&self, other: T, base: T) -> T;

  /// Subtract another unsigned integer and start from a specified base 
  /// if the target result may be otherwise be negative if it were a signed integer
  /// e.g. if a base of 24 is applied to a u8 value, its max is 23
  /// 3.cycle_sub(4) is thus 23
  fn cycle_sub(&self, other: T, base: T) -> T;

  /// Add another unsigned integer representing a 
  /// number in a series from 1 to specified maximum
  /// Unlike cycle_add the max value is the limit 
  /// and the minimum value is 1. Hence 1.series_add(7, 1) will be 1
  fn series_add(&self, other: T, limit: T) -> T;

  /// Subtract another unsigned integer representing a 
  /// number in a series from 1 to specified maximum
  /// Unlike cycle_sub the max value is the limit 
  /// and the minimum value is 1. Hence 1.series_sub(1, 7) will be 7
  fn series_sub(&self, other: T, limit: T) -> T;

  /// Find the remainder in a range between one and a limit.
  /// This methods normalises an unsigned integer within a series  
  /// from 1 to a specified limit
  /// Unlike % or T.mod(T) 0.series_mod(&T) will equal the limit as results cannot be lower than 1
  /// This is mainly used for arbitrary series like weekday numbers 1 to 7 or month numbers 1 to 31
  /// The specified limit may be no higher than the unsigned type's MAX
  /// e.g. 255 for u8
  fn series_mod(&self,  limit: T) -> T;

}

/// Macro to implement the above for u8, u16 and u32
macro_rules! impl_cycle_add {
    ($t:ty,$u:ty) => {
      impl CycleAdd<$t> for $t {
        fn cycle_add(&self, other: $t, base: $t) -> $t {
          let b2 = base as $u;
          let result: $u = *self as $u + other as $u;
          if result < b2 {
            result as $t
          } else {
            (result % base as $u) as $t
          }
        }

        fn cycle_sub(&self, other: $t, base: $t) -> $t {
            if *self < other {
              let b2 = base as $u;
              let result = b2 + *self as $u - other as $u ;
              (result % b2) as $t
            } else {
                *self - (other % base)
            }
        }

        fn series_add(&self, other: $t, limit: $t) ->  $t {
          let start_val = *self  - 1;
          let b2 = limit as $u;
          let result: $u = start_val as $u + other as $u;
          if result < b2 {
            result as $t + 1
          } else {
            (result % limit as $u) as $t + 1
          }
        }


        fn series_sub(&self, other: $t, limit: $t) ->  $t {
          let start_val = *self  - 1;
          if start_val < other {
            let b2 = limit as $u;
            let result = b2 + start_val as $u - other as $u ;
            (result % b2) as $t + 1
          } else {
              start_val - (other % limit) + 1
          }
        }

        fn series_mod(&self, limit: $t) ->  $t {
          let result = *self % limit;
          if result < 1 {
            limit
          } else {
            result
          }
        }
      }
    };
}

// Implement for u8, max 255
impl_cycle_add!(u8, u16);

// Implement for u16, max 65535
impl_cycle_add!(u16, u32);

// Implement for u32, max 4294967295
impl_cycle_add!(u32, u64);
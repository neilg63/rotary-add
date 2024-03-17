
mod rotary;
mod cycle;

/// RotaryAdd works on the whole range of the unsigned integer
pub use rotary::*;

/// CycleAdd works on range from 0 to a a specified base. The max value is 1 less than the base
pub use cycle::*;

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


  #[test]
  fn test_cycle_add_u8() {
    let num_1: u8 = 35;
    let num_2: u8 = 29;
    let result = num_1.cycle_add(&num_2, 60);
    let expected = 4;
    assert_eq!(result, expected);
  }

  #[test]
  fn test_cycle_sub_u16() {
    let num_1: u16 = 20_000;
    let num_2: u16 = 24_000;
    let base: u16 = 16_384;
    let result = num_1.cycle_sub(&num_2, base);
    let expected = 12384;
    assert_eq!(result, expected);
  }

}

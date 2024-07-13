#[cfg(test)]

use rotary_add::*;

#[test]
fn test_rotary_add_u8() {
  // 254 + 37 with rotary logic applied to u8 (2^8) should overflow to 35
  let num_1: u8 = 254;
  let num_2: u8 = 37;
  let result = num_1.rotary_add(&num_2);
  let expected_result = 35;
  assert_eq!(result, expected_result);

  // 4 subtracted from 3 with rotary logic applied to u8 should overflow to 255
  // i.e. 2^8 - 1
  let result_2: u8 = 3.rotary_sub(&4);
  let expected_result_2 = 255;
  assert_eq!(result_2, expected_result_2);
}

#[test]
fn test_rotary_add_u32() {
  let num_1: u32 = 3_998_373_629;
  let num_2: u32 = 998_373_673;
  let result = num_1.rotary_add(&num_2);
  let expected_result = 701780006;
  assert_eq!(result, expected_result);
}

#[test]
fn test_rotary_add_u16() {
  let num_1: u16 = 64_777;
  let num_2: u16 = 1_000;
  let result = num_1.rotary_add(&num_2);
  let expected_result = 241;
  assert_eq!(result, expected_result);
}

#[test]
fn test_rotary_sub_u8() {
  // subtracting a larger number will not fail
  // but count backwards from the max value
  let num_1: u8 = 127;
  let num_2: u8 = 128;
  let result = num_1.rotary_sub(&num_2);
  let expected = 255;
  assert_eq!(result, expected);

  // normal subtraction without overflow should work as expected
  let num_1: u8 = 137;
  let num_2: u8 = 128;
  let result = num_1.rotary_sub(&num_2);
  let expected = 9;
  assert_eq!(result, expected);

  // subtracting a larger number will not fail
  // but count backwards from the max value
  let num_1: u8 = 3;
  let num_2: u8 = 240;
  let result = num_1.rotary_sub(&num_2);
  let expected = 19;
  assert_eq!(result, expected);

  // subtracting the same value should always yield zero
  let num_1: u8 = 143;
  let num_2: u8 = 143;
  let result = num_1.rotary_sub(&num_2);
  let expected = 0;
  assert_eq!(result, expected);
}

#[test]
fn test_rotary_sub_u32() {

  // subtracting a larger number will not fail
  // but count backwards from the max value
  let num_1: u32 = 998_373_673;
  let num_2: u32 = 3_998_373_629;
  let result = num_1.rotary_sub(&num_2);
  let expected = 1_294_967_340;
  assert_eq!(result, expected);

  // A notional minus one value (impossible with unsigned integers)
  // should yield the maximum value, which for u32 is 2 ^ 32 - 1
  let num_1: u32 = 2_998_373_673;
  let num_2: u32 = 2_998_373_674; // one more
  let result = num_1.rotary_sub(&num_2);
  let expected = 4_294_967_295; // u32::MAX
  assert_eq!(result, expected);
}


#[test]
fn test_cycle_add_u8() {
  // Addition of 35 and 29 with base 60 should overflow to 4
  let num_1: u8 = 35;
  let num_2: u8 = 29;
  let result = num_1.cycle_add(&num_2, 60);
  let expected = 4;
  assert_eq!(result, expected);
}

#[test]
fn test_cycle_sub_u16() {
  // Subtracting 24,000 from 20,000 with base 16,384 (2 ^ 14) )hould overflow to 12,384
  let num_1: u16 = 20_000;
  let num_2: u16 = 24_000;
  let base: u16 = 16_384;
  let result = num_1.cycle_sub(&num_2, base);
  let expected = 12_384;
  assert_eq!(result, expected);
}


#[test]
fn test_series_add_u8() {
  // Adding 6 + 2 with a series of 1 to 7 (e.g. weekdays) should yield 1
  let num_1: u8 = 6;
  let num_2: u8 = 2;
  let result = num_1.series_add(&num_2, 7);
  let expected = 1;
  assert_eq!(result, expected);
}

#[test]
fn test_series_sub_u8() {
  // Subtracting 6 from 2 with a series of 1 to 7 (e.g. weekdays) should yield 3
  let num_1: u8 = 2;
  let num_2: u8 = 6;
  let result = num_1.series_sub(&num_2, 7);
  let expected = 3;
  assert_eq!(result, expected);
}


#[test]
fn test_series_mod_u8() {
  // 10 mod 7 should equal 3 with a series of 1 to 7
  let num_1: u8 = 10;
  let result = num_1.series_mod(7);
  let expected = 3;
  assert_eq!(result, expected);

  // 7 mod 7 should equal 7 with a series of 1 to 7
  // with a 0-based index, it would equal 0
  let num_2: u8 = 7;
  let result = num_2.series_mod(7);
  let expected = 7;
  assert_eq!(result, expected);
}
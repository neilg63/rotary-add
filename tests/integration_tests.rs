#[cfg(test)]

use rotary_add::*;

#[test]
fn test_cycle_add_u8() {
  // Addition of 35 and 29 with base 60 should overflow to 4
  let num_1: u8 = 35;
  let num_2: u8 = 29;
  let result = num_1.cycle_add(num_2, 60);
  let expected = 4;
  assert_eq!(result, expected);
}

#[test]
fn test_cycle_sub_u16() {
  // Subtracting 24,000 from 20,000 with base 16,384 (2 ^ 14) ) should overflow to 12,384
  let num_1: u16 = 20_000;
  let num_2: u16 = 24_000;
  let base: u16 = 16_384;
  let result = num_1.cycle_sub(num_2, base);
  let expected = 12_384;
  assert_eq!(result, expected);
}

#[test]
fn test_cycle_add_u32() {
  // Subtracting 24,000 from 20,000 with base 16,384 (2 ^ 14) ) should overflow to 12,384
  let num_1: u32 = 15_000_000;
  let num_2: u32 = 4_000_000;
  let base: u32 = 16_777_216; // 2 ^ 14
  let result = num_1.cycle_add(num_2, base);
  let expected = 2222784;
  assert_eq!(result, expected);
}


#[test]
fn test_series_add_u8() {
  // Adding 6 + 2 with a series of 1 to 7 (e.g. weekdays) should yield 1
  let num_1: u8 = 6;
  let num_2: u8 = 2;
  let result = num_1.series_add(num_2, 7);
  let expected = 1;
  assert_eq!(result, expected);
}

#[test]
fn test_series_sub_u8() {
  // Subtracting 6 from 2 with a series of 1 to 7 (e.g. weekdays) should yield 3
  let num_1: u8 = 2;
  let num_2: u8 = 6;
  let result = num_1.series_sub(num_2, 7);
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
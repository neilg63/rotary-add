[![Mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/rotary-add)
[![Crates.io](https://img.shields.io/crates/v/rotary-add.svg)](https://crates.io/crates/rotary-add)
[![Docs.rs](https://docs.rs/rotary-add/badge.svg)](https://docs.rs/rotary-add)

# Cyclical Arithmetic with Unsigned Integers and Arbitrary Ranges

This library crate provides an extension trait, `CycleAdd`, for performing cyclical (modular) addition and subtraction with unsigned integer types (`u8`, `u16`, and `u32`). Unlike the default `+` and `-` operators, these operations never overflow below zero or beyond the defined modulus.

## Features

- **Cyclical addition and subtraction**: Perform modular arithmetic without overflow.
- **1-based series support**: Simplifies operations on series starting from 1 (e.g., months, weekdays).
- **Lightweight and focused**: Extends core unsigned integer types for cryptography, metrology, and other use cases.

## Examples

### Cyclical Addition
```rust
let first_number: u8 = 22;
let second_number: u8 = 6;

let result = first_number.cycle_add(second_number, 24);
// Result: 4 (22 + 6 wraps around at 24)
```

### Cyclical Subtraction
```rust
let first_number: u8 = 37;

let result = first_number.cycle_sub(78, 100);
// Result: 59 (37 - 78 wraps around at 100)
```

### 1-Based Series Operations
Many common series (e.g., months, weekdays) start from 1. Modular arithmetic typically assumes 0-based indexing, which can lead to unexpected results. The `series_add`, `series_sub`, and `series_mod` methods handle these conversions for you.

```rust
let sample_month_1: u8 = 11;
let limit = 12; // Months of the year

let result = sample_month_1.series_add(1, limit);
// Result: 12 (November + 1 = December)

let sample_month_2: u8 = 3;
let result = sample_month_2.series_sub(3, limit);
// Result: 12 (March - 3 = December)

let sample_month_value: u8 = 24; // 24th month
let result = sample_month_value.series_mod(limit);
// Result: 12 (24 wraps around to December)
```

## Comparison with Related Crates

Unlike the [Ring360](https://crates.io/crates/ring360) crate, `RotaryAdd` focuses exclusively on extending core unsigned integer types. It is designed for use cases such as:

- Cryptography
- Metrology
- Building blocks for other crates (e.g., encoding/decoding operations)

### Changelog

#### Version 0.2.1
- Corrections to README.md and tests only.

#### Version 0.2.0
- Removed `T.rotary_add(&T)` and `T.rotary_sub(&T)` as they were functionally equivalent to `T.wrapping_add(T)` and `T.wrapping_sub(T)` in the Rust core library (but with the first parameter passed by reference).
- All remaining extension methods now take the first parameter by value, consistent with `wrapping_add` and `wrapping_sub`.

#### Version 0.1.2
- Added `series_add`, `series_sub`, and `series_mod` methods for 1-based series operations.

---

For more details, visit the [documentation](https://docs.rs/rotary-add) or the [GitHub repository](https://github.com/neilg63/rotary-add).
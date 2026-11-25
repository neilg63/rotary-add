[![Mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/rotary-add)
[![Crates.io](https://img.shields.io/crates/v/rotary-add.svg)](https://crates.io/crates/rotary-add)
[![Docs.rs](https://docs.rs/rotary-add/badge.svg)](https://docs.rs/rotary-add)

# Rotary Add: cyclical arithmetic for unsigned integers

`rotary-add` provides one extension trait, `CycleAdd`, for `u8`, `u16` and `u32`. It adds and subtracts within a cycle that you choose, such as hours, minutes, months, weekdays or compass degrees. Results wrap around at the cycle's limit instead of overflowing or underflowing.

The trait supports two kinds of cycle:

| Cycle type | Values | Methods | Typical uses |
|------------|--------|---------|--------------|
| **0-based** | `0` to `base - 1` | `cycle_add`, `cycle_sub` | 24-hour clock (0–23), minutes (0–59), degrees (0–359), array indices |
| **1-based** | `1` to `limit` | `series_add`, `series_sub`, `series_mod` | months (1–12), weekdays (1–7), 12-hour clock (1–12), days of the month |

In both cases, the second argument is the **number of values in the cycle**: `24` for hours, `12` for months, `7` for weekdays. In a 0-based cycle the highest value is one less than this number. In a 1-based cycle the highest value equals it.

## Installation

```toml
[dependencies]
rotary-add = "0.2"
```

```rust
use rotary_add::CycleAdd;
```

## 0-based cycles: `cycle_add` and `cycle_sub`

Use these when the cycle starts at zero. They work like `(a + b) % base`, but `cycle_sub` never underflows.

```rust
use rotary_add::CycleAdd;

// 24-hour clock: 22:00 + 6 hours = 04:00
assert_eq!(22u8.cycle_add(6, 24), 4);

// 24-hour clock: 03:00 - 5 hours = 22:00
assert_eq!(3u8.cycle_sub(5, 24), 22);

// Minutes: 50 + 25 minutes = 15 minutes past the next hour
assert_eq!(50u8.cycle_add(25, 60), 15);

// Compass bearings (u16, since 359 does not fit in a u8)
assert_eq!(350u16.cycle_add(20, 360), 10);
assert_eq!(10u16.cycle_sub(20, 360), 350);
```

## 1-based cycles: `series_add`, `series_sub` and `series_mod`

Many everyday sequences start at 1. With plain modular arithmetic, December (12) plus one month gives `13 % 12 = 1`, which is correct. But December itself gives `12 % 12 = 0`, which is not a month. The `series_*` methods keep every result between `1` and `limit`.

```rust
use rotary_add::CycleAdd;

// Months (1 = January, 12 = December)
assert_eq!(11u8.series_add(1, 12), 12); // November + 1 = December
assert_eq!(12u8.series_add(1, 12), 1);  // December + 1 = January
assert_eq!(3u8.series_sub(3, 12), 12);  // March - 3 = December

// Weekdays (1 = Monday, 7 = Sunday)
assert_eq!(6u8.series_add(2, 7), 1);    // Saturday + 2 days = Monday
assert_eq!(2u8.series_sub(6, 7), 3);    // Tuesday - 6 days = Wednesday

// 12-hour clock: 11 o'clock + 3 hours = 2 o'clock
assert_eq!(11u8.series_add(3, 12), 2);
```

`series_mod` maps any value onto the 1-based cycle. It works like `%`, except that an exact multiple of the limit returns the limit rather than `0`:

```rust
use rotary_add::CycleAdd;

assert_eq!(13u8.series_mod(12), 1);  // month 13 -> January
assert_eq!(24u8.series_mod(12), 12); // month 24 -> December (not 0)
assert_eq!(0u8.series_mod(12), 12);  // 0 -> December
```

### How the two kinds relate

For a value `x` in `1..=limit`, the 1-based methods give the same result as shifting down to 0-based, applying the 0-based method, and shifting back up:

```rust
use rotary_add::CycleAdd;

let (x, n, limit) = (5u8, 9u8, 7u8);
assert_eq!(x.series_add(n, limit), (x - 1).cycle_add(n, limit) + 1);
assert_eq!(x.series_sub(n % limit, limit), (x - 1).cycle_sub(n % limit, limit) + 1);
```

If your data uses 1-based numbering, use the `series_*` methods and avoid converting by hand.

## Input ranges

These methods do not check their inputs. Keep inputs within the ranges below. Outside them, subtraction can underflow, which panics in debug builds and gives a wrong result in release builds.

| Method | Receiver (`self`) | Offset (`other`) | Result |
|--------|-------------------|------------------|--------|
| `cycle_add(other, base)` | any | any | `0..base` |
| `cycle_sub(other, base)` | `0..base` | `0..=base` | `0..base` |
| `series_add(other, limit)` | `1..` (not `0`) | any | `1..=limit` |
| `series_sub(other, limit)` | `1..=limit` | `0..=limit` | `1..=limit` |
| `series_mod(limit)` | any | — | `1..=limit` |

`base` and `limit` must be greater than `0`.

To subtract an offset that may be larger than the cycle, reduce it first:

```rust
use rotary_add::CycleAdd;

let days_back: u16 = 100;
assert_eq!(3u16.series_sub(days_back % 7, 7), 1); // Wednesday - 100 days = Monday
```

To bring a 0-based value into range before subtracting, use `% base`. For a 1-based value, use `series_mod(limit)`.

## Why not `wrapping_add` or `%`?

- `wrapping_add` and `wrapping_sub` wrap only at the type's own size (256 for `u8`, 65,536 for `u16`, and so on). They cannot wrap at 24, 60 or 360.
- `(a + b) % base` can overflow if `a + b` exceeds the type's maximum. `cycle_add` adds in the next larger type (`u8` → `u16`, `u16` → `u32`, `u32` → `u64`), so it does not overflow.
- `(a - b) % base` underflows whenever `b > a`. `cycle_sub` handles this case.
- `%` cannot produce a 1-based result. The `series_*` methods can.

## Related crates

[Ring360](https://crates.io/crates/ring360) handles floating-point angles on a 360° circle. `rotary-add` only extends the core unsigned integer types. It is intended for:

- Calendar and clock arithmetic
- Cryptography and encoding/decoding, where values cycle within a fixed alphabet or block size
- Metrology
- Building blocks for other crates

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Changelog

### 0.2.2
- Relicensed under MIT OR Apache-2.0. Versions 0.2.1 and earlier were published under GPL-2.0-or-later WITH Bison-exception-2.2.
- Rewrote README.md to explain 0-based and 1-based cycles and the valid input ranges. No functional changes.

### 0.2.1
- Corrections to README.md, comments and tests only.

### 0.2.0
- Removed `rotary_add` and `rotary_sub`. They did the same as the core library's `wrapping_add` and `wrapping_sub`, except that they took the other operand by reference.
- The `other`, `base` and `limit` arguments of the remaining methods are now passed by value, like the arithmetic methods in the standard library.

### 0.1.2
- Added `series_add`, `series_sub` and `series_mod` for 1-based cycles.

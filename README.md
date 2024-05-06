[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/rotary-add)
[![crates.io](https://img.shields.io/crates/v/rotary-add.svg)](https://crates.io/crates/rotary-add)
[![docs.rs](https://docs.rs/rotary-add/badge.svg)](https://docs.rs/rotary-add)

# RotaryAdd: Cyclical arithmetic with unsigned integers

This crate provides 3 traits, ```RotaryAdd```,  ```CycleAdd``` and ```SeriesAdd```, with simple methods to apply cyclical or modular addition and subtraction with three unsigned integer types, u8, u16 and u32. Unlike the default ```+``` and ```-``` operators, additions and subtractions never overflow.

## RotaryAdd

This trait has addition and subtraction methods that act on the whole range of an unsigned integer, e.g. from *0 to 255* for *u8* or from *0 to 65535* for *u16*.
This example with an 8-bit unsigned integer illustrates the concept with the ```rotary_add``` and ```rotary_sub``` methods:

Addition:
```rust
let first_number: u8 = 255;
let second_number: u8 = 6;

let result = first_number.rotary_add(&second_number);
// yields 5. first_number + second_number would panic as the value would overflow
```

Subtraction

```rust
let first_number: u8 = 3;
let second_number: u8 = 6;
let result = first_number.rotary_sub(&second_number);
// yields 253. first_number - second_number would panic as the value would overflow
```

## CycleAdd

This trait provides addition and subtraction methods that let you define a custom *cyclic base*.

These examples show the concept:

#### Addition:
```rust
let first_number: u8 = 22;
let second_number: u8 = 6;

let result = first_number.cycle_add(&second_number, 24);
// yields 4. We have to use u8 as a primitive data type, but
```

#### Subtraction

```rust
let first_number: u8 = 37;
let second_number: u8 = 78;
let result = first_number.cycle_sub(&second_number, 100);
// yields 59. first_number - second_number would panic as the value would overflow
```

#### Serial addition, subtraction and modulus, starting from 1
Many common series start from one. The days of the month and week are commonly expressed with the numerals 1 to the maximum. Alas this does not work with modular arithmetic where additions or subtractions overflow. If 1 means Monday and 7 means Sunday, we'd expect *2 (Tue) - 3* to equal 7 (Sun) and *6 (Sat) + 1* to equal 7 (Sun). Instead we need to subtract one from the input number, that may be no lower than 1, mod it by the maximum number and then add 1 to the result. The series_add(), series_sub() and series_mod() methods avoid the need for these conversions when dealing with 1-based serial inputs.
```rust
let sample_month_1: u8 = 11;
let limit = 12; // months of the year
let result = sample_month_1.series_add(1, limit);
// yields 12. 
let sample_month_2: u8 = 3;
let result = sample_month_2.series_sub(3, limit);
// yields 12

let sample_month_value: u8 = 24; // 24th month
let result = sample_month_value.series_mod(limit);
// yields 12, but would yield 0 with 0-indexed modulus

```
Unlike the related [Ring360](https://crates.io/crates/ring360) crate, this library only extends core unsigned integer types for use with cryptography and as a building block for other crates, e.g. converting characters first to u32 values and then shifting their values in one direction in the encoding stage and reversing the process in the decoding stage. 

## Dev notes
This is an alpha release.

### 0.1.2
- Added series_add(), series_sub(), series_mod() methods.
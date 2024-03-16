[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/rotary-add)
[![crates.io](https://img.shields.io/crates/v/rotary-add.svg)](https://crates.io/crates/rotary-add)
[![docs.rs](https://docs.rs/rotary-add/badge.svg)](https://docs.rs/rotary-add)

# RotaryAdd: Cyclical arithemtic with unsigned integers

This crate provides a trait with two simple methods, rotary_add() and rotary_sub(), to apply cyclical or modular arithmetic with three unsigned integer types, u8, u16 and u32. Unlike the default + and - operators, additions and subtractions never overflow. This example with an 8-bit unsigned integer illustrates the difference:

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

Unlike the related [Ring360](https://crates.io/crates/ring360) crate, this library only extends core unsigned integer types for use with cryptography and as a building block for other crates, e.g. converting characters first to u32 values and then shifting their values in one direction in the encoding stage and reversing the process in the decoding stage. 

## Dev notes
This is an alpha release.
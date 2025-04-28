
mod cycle;

/// CycleAdd works on range from 0 to a a specified modulus. The max value is 1 less than the modulus
/// with a 0-based system (e.g. minutes in an hour) 
/// and the same as the modulus with a 1-based system (e.g. months in a year).

pub use cycle::*;

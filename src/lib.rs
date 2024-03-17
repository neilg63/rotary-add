
mod rotary;
mod cycle;

/// RotaryAdd works on the whole range of the unsigned integer
pub use rotary::*;

/// CycleAdd works on range from 0 to a a specified base. The max value is 1 less than the base
pub use cycle::*;

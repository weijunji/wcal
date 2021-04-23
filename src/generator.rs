//! Use ast to calculate or generate target code.
//!
//! Current implemented generator:
//! * `calculator`: calculate the expression to `i128`, will
//! cause a cast in division
//! * `calculator_f`: calculate the expression to `f64`

pub mod calculator;
pub mod calculator_f;

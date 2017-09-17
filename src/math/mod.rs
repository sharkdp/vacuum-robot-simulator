use std::f64::consts;

/// Underlying numeric type.
pub type Scalar = f64;

/// PI as a Scalar
pub const PI: Scalar = consts::PI;

/// Type synonym for angles.
pub type Angle = Scalar;

use num::{Float, clamp, NumCast};

pub fn nonzero<F>(value: F) -> F
where
    F: Float,
{
    use std::f64::{MIN_POSITIVE, MAX};
    clamp(
        value,
        NumCast::from(MIN_POSITIVE).unwrap(),
        NumCast::from(MAX).unwrap(),
    )
}

pub fn normal<F>(value: F) -> F
where
    F: Float,
{
    use num::{Zero, One};
    clamp(value, Zero::zero(), One::one())
}
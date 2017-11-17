use num::{clamp, NumCast, Zero};

pub trait NonZero {
    fn nonzero(self) -> Self;
}

impl NonZero for f64 {
    fn nonzero(self) -> Self {
        use std::f64::{MIN_POSITIVE, MAX};
        clamp(
            self,
            NumCast::from(MIN_POSITIVE).unwrap(),
            NumCast::from(MAX).unwrap(),
        )
    }
}
impl NonZero for f32 {
    fn nonzero(self) -> Self {
        use std::f32::{MIN_POSITIVE, MAX};
        clamp(
            self,
            NumCast::from(MIN_POSITIVE).unwrap(),
            NumCast::from(MAX).unwrap(),
        )
    }
}

impl NonZero for i8 {
    fn nonzero(self) -> Self {
        use std::i8::MAX;
        clamp(self, 1, MAX)
    }
}

impl NonZero for i16 {
    fn nonzero(self) -> Self {
        use std::i16::MAX;
        clamp(self, 1, MAX)
    }
}

impl NonZero for i32 {
    fn nonzero(self) -> Self {
        use std::i32::MAX;
        clamp(self, 1, MAX)
    }
}

impl NonZero for i64 {
    fn nonzero(self) -> Self {
        use std::i64::MAX;
        clamp(self, 1, MAX)
    }
}

pub trait Normal {
    fn normal(self) -> Self;
}

impl Normal for f64 {
    fn normal(self) -> Self {
        use num::{Zero, One};
        clamp(self, Zero::zero(), One::one())
    }
}

impl Normal for f32 {
    fn normal(self) -> Self {
        use num::{Zero, One};
        clamp(self, Zero::zero(), One::one())
    }
}

impl Normal for i8 {
    fn normal(self) -> Self {
        use std::i8::MAX;
        clamp(self, Zero::zero(), MAX)
    }
}

impl Normal for i16 {
    fn normal(self) -> Self {
        use std::i16::MAX;
        clamp(self, Zero::zero(), MAX)
    }
}

impl Normal for i32 {
    fn normal(self) -> Self {
        use std::i32::MAX;
        clamp(self, Zero::zero(), MAX)
    }
}

impl Normal for i64 {
    fn normal(self) -> Self {
        use std::i64::MAX;
        clamp(self, Zero::zero(), MAX)
    }
}

use num::{clamp, NumCast, Zero};

pub trait NonZero {
    fn nonzero(self) -> Self;
}

macro_rules! impl_nonzero_float {
    ($($t: tt), *) => {
        $(impl NonZero for $t {
            fn nonzero(self) -> Self {
                use std::$t::{MIN_POSITIVE, MAX};
                clamp(
                    self,
                    NumCast::from(MIN_POSITIVE).unwrap(),
                    NumCast::from(MAX).unwrap(),
                )
            }
        })*
    };
}
macro_rules! impl_nonzero_signed {
    ($($t: tt), *) => {
        $(impl NonZero for $t {
            fn nonzero(self) -> Self {
                use std::$t::MAX;
                clamp(
                    self,
                    1,
                    MAX,
                )
            }
        })*
    };
}
impl_nonzero_float!(f64, f32);
impl_nonzero_signed!(i8, i16, i32, i64, isize);


pub trait Normal {
    fn normal(self) -> Self;
}

macro_rules! impl_normal_float {
    ($($t: tt), *) => {
        $(impl Normal for $t {
            fn normal(self) -> Self {
                use num::{Zero, One};
                clamp(self, Zero::zero(), One::one())
            }
        })*
    }
}

macro_rules!  impl_normal_signed{
    ($($t: tt), *) => {
        $(impl Normal for $t {
            fn normal(self) -> Self {
                use std::$t::MAX;
                clamp(self, Zero::zero(), MAX)
            }
        })* 
    };
}

impl_normal_float!(f64, f32);
impl_normal_signed!(i8, i16, i32, i64, isize);

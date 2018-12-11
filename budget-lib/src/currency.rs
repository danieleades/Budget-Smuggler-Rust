use decimal::d128;
use std::{cmp, ops};

pub trait Currency:
    Default + cmp::PartialEq + ops::AddAssign + ops::Mul + ops::SubAssign + Copy
{
}

impl Currency for d128 {}
impl Currency for i8 {}
impl Currency for i16 {}
impl Currency for i32 {}
impl Currency for i64 {}
impl Currency for i128 {}

impl Currency for u8 {}
impl Currency for u16 {}
impl Currency for u32 {}
impl Currency for u64 {}
impl Currency for u128 {}

impl Currency for f32 {}
impl Currency for f64 {}

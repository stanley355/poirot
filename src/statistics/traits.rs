use std::ops::{Add, Div};

pub trait StaticNumber: Copy + Add<Output = Self> + Div<Output = Self> {}
impl<T> StaticNumber for T where T: Copy + Add<Output = T> + Div<Output = T> {}

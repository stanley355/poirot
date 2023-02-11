use std::ops::{Add, Div};

pub trait MeanNumbers: Copy + Add<Output = Self> + Div<Output = Self> {}
impl<T> MeanNumbers for T where T: Copy + Add<Output = T> + Div<Output = T> {}

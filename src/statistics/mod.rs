pub mod mean;
pub mod traits;

pub struct Statistics;

impl Statistics {
    pub fn calc_integer_mean<T>(numbers: &Vec<T>) -> T
    where
        T: traits::MeanNumbers + From<i32>,
    {
        mean::calc_integer_mean(numbers)
    }

    pub fn calc_float_mean<T>(numbers: &Vec<T>) -> T
    where
        T: traits::MeanNumbers + From<f32>,
    {
        mean::calc_float_mean(numbers)
    }
}

use self::traits::MeanNumbers;

pub mod mean;
pub mod traits;
pub mod tests;

pub struct Statistics;

impl Statistics {
    pub fn calc_integer_mean<T: MeanNumbers>(numbers: &Vec<T>) -> T
    where
        T: traits::MeanNumbers + From<i32>,
    {
        mean::calc_integer_mean(numbers)
    }

    pub fn calc_float_mean<T: MeanNumbers>(numbers: &Vec<T>) -> T
    where
        T: traits::MeanNumbers + From<f32>,
    {
        mean::calc_float_mean(numbers)
    }
}

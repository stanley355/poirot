use self::traits::StaticNumber;

pub mod mean;
pub mod traits;
pub mod tests;

pub struct Statistics;

impl Statistics {
    pub fn calc_integer_mean<T: StaticNumber>(numbers: &Vec<T>) -> T
    where
        T: traits::StaticNumber + From<i32>,
    {
        mean::calc_integer_mean(numbers)
    }

    pub fn calc_float_mean<T: StaticNumber>(numbers: &Vec<T>) -> T
    where
        T: traits::StaticNumber + From<f32>,
    {
        mean::calc_float_mean(numbers)
    }
}

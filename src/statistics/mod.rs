use self::traits::StaticNumber;

pub mod absolute;
pub mod mean;
pub mod tests;
pub mod traits;

pub struct Statistics;

impl Statistics {
    pub fn calc_integer_mean<T: StaticNumber>(numbers: &Vec<T>) -> T
    where
        T: From<i32>,
    {
        mean::calc_integer_mean(numbers)
    }

    pub fn calc_float_mean<T: StaticNumber>(numbers: &Vec<T>) -> T
    where
        T: From<f32>,
    {
        mean::calc_float_mean(numbers)
    }

    pub fn max<T: Ord>(numbers: &Vec<T>) -> &T
    where
        T: Ord,
    {
        absolute::max(numbers)
    }
}

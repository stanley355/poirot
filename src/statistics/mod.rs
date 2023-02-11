pub mod mean;
pub mod traits;

pub struct Statistics;

impl Statistics {
    pub fn calc_integer_mean<T>(numbers: &Vec<T>) -> T
    where
        T: traits::MeanNumbers + From<i32>,
    {
        mean::mean_i32(numbers)
    }
}

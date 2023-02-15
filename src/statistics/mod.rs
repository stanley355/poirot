use num::Float;

pub mod absolute;
pub mod mean;
pub mod tests;

pub struct Statistics;

impl Statistics {
    pub fn calc_float_mean<F: Float>(numbers: &Vec<F>) -> F
    where
        F: Float,
    {
        
        mean::calc_mean(numbers)
    }

    pub fn max<T: Ord>(numbers: &Vec<T>) -> &T
    where
        T: Ord,
    {
        absolute::max(numbers)
    }
}

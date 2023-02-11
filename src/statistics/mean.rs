use super::traits::MeanNumbers;

pub fn calc_integer_mean<T: MeanNumbers>(numbers: &Vec<T>) -> T
where
    T: MeanNumbers + From<i32>,
{
    let sum: T = numbers.iter().fold(T::from(0), |acc, &x| acc + x);
    sum / T::from(numbers.len() as i32)
}

pub fn calc_float_mean<T: MeanNumbers>(numbers: &Vec<T>) -> T
where
    T: MeanNumbers + From<f32>,
{
    let sum: T = numbers.iter().fold(T::from(0.0), |acc, &x| acc + x);
    sum / T::from(numbers.len() as f32)
}

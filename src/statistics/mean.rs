use super::traits::StaticNumber;

pub fn calc_integer_mean<T: StaticNumber>(numbers: &Vec<T>) -> T
where
    T: StaticNumber + From<i32>,
{
    let sum: T = numbers.iter().fold(T::from(0), |acc, &x| acc + x);
    sum / T::from(numbers.len() as i32)
}

pub fn calc_float_mean<T: StaticNumber>(numbers: &Vec<T>) -> T
where
    T: StaticNumber + From<f32>,
{
    let sum: T = numbers.iter().fold(T::from(0.0), |acc, &x| acc + x);
    sum / T::from(numbers.len() as f32)
}

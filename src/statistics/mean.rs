use super::traits::StaticNumber;

pub fn calc_mean<T: StaticNumber>(numbers: &Vec<T>) -> T
where
    T: StaticNumber + From<f32>,
{
    let sum: T = numbers.iter().fold(T::from(0.0), |acc, &x| acc + x);
    sum / T::from(numbers.len() as f32)
}

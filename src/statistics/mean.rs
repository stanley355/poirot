use num::Float;

pub fn calc_mean<F: Float>(numbers: &Vec<F>) -> F
where
    F: Float
{
    let sum: F = numbers.iter().fold(F::from(0.0).unwrap(), |acc, &x| acc + x);
    sum / F::from(numbers.len() as f32).unwrap()
}

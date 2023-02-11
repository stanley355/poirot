use super::traits::MeanNumbers;

pub fn calc_integer_mean<T>(numbers: &Vec<T>) -> T
where
    T: MeanNumbers + From<i32>,
{
    let sum: T = numbers.iter().fold(T::from(0), |acc, &x| acc + x);
    sum / T::from(numbers.len() as i32)
}

pub fn calc_float_mean<T>(numbers: &Vec<T>) -> T
where
    T: MeanNumbers + From<f32>,
{
    let sum: T = numbers.iter().fold(T::from(0.0), |acc, &x| acc + x);
    sum / T::from(numbers.len() as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_mean() {
        let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
        let result = calc_integer_mean(&number_list);
        assert_eq!(result, 54);
    }

    #[test]
    fn test_float_mean() {
        let number_list: Vec<f32> = vec![34.0, 50.0, 25.0, 100.0, 65.0];
        let result = calc_float_mean(&number_list);
        assert_eq!(result, 54.8);
    }
}

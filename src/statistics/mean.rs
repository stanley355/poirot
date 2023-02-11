use super::traits::MeanNumbers;

pub fn mean_i32<T>(numbers: &Vec<T>) -> T
where
    T: MeanNumbers + From<i32>,
{
    let sum: T = numbers.iter().fold(T::from(0), |acc, &x| acc + x);
    sum / T::from(numbers.len() as i32)
}

pub fn mean_f32<T>(numbers: &Vec<T>) -> T
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
    fn test_mean_i32() {
        let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];

        let result = mean_i32(&number_list);
        assert_eq!(result, 54);
    }
}

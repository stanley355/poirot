use core::num;

use std::ops::{Add, Div};

pub fn mean<T>(numbers: &Vec<T>) -> T
where
    T: Copy + Add<Output = T> + Div<Output = T> + From<u32>,
{
    let sum: T = numbers.iter().fold(T::from(0), |acc, &x| acc + x);
    sum / T::from(numbers.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_u32() {
        let number_list: Vec<u32> = vec![34, 50, 25, 100, 65];

        let result = mean(&number_list);
        assert_eq!(result, 54);
    }
}

use num::Float;

pub fn mean<F: Float>(numbers: &Vec<F>) -> F
where
    F: Float,
{
    let sum: F = numbers
        .iter()
        .fold(F::from(0.0).unwrap(), |acc, &x| acc + x);
    sum / F::from(numbers.len() as f32).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean_f32() {
        let number_list: Vec<f32> = vec![34.0, 50.0, 25.0, 100.0, 65.0];
        let result = mean(&number_list);
        assert_eq!(result, 54.8);
    }
    
    #[test]
    fn test_mean_f64() {
        let number_list: Vec<f64> = vec![34.0, 50.0, 25.0, 100.0, 65.0];
        let result = mean(&number_list);
        assert_eq!(result, 54.8);
    }
    
    #[test]
    fn test_mean_with_more_data() {
        let number_list: Vec<f64> = vec![34.0, 50.0, 25.0, 100.0, 65.0, 34.0, 50.0, 25.0, 100.0, 65.0];
        let result = mean(&number_list);
        assert_eq!(result, 54.8);
    }
    
    #[test]
    fn test_mean_with_varied_data() {
        let number_list: Vec<f64> = vec![34.0, 50.0, 25.0, 100.0, 65.0, 43.0, 5.0, 52.0, 10.0, 56.0];
        let result = mean(&number_list);
        assert_eq!(result, 44.0);
    }
}

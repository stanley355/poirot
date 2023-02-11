#[allow(unused_imports)]
use super::Statistics;

#[cfg(test)]
mod tests {
    use super::Statistics;

    #[test]
    fn test_integer_mean() {
        let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
        let result = Statistics::calc_integer_mean(&number_list);
        assert_eq!(result, 54);
    }

    #[test]
    fn test_float_mean() {
        let number_list: Vec<f32> = vec![34.0, 50.0, 25.0, 100.0, 65.0];
        let result = Statistics::calc_float_mean(&number_list);
        assert_eq!(result, 54.8);
    }
}
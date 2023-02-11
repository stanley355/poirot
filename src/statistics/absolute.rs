pub fn max<T: Ord>(numbers: &Vec<T>) -> &T
where
    T: Ord,
{
    numbers.iter().max().unwrap()
}

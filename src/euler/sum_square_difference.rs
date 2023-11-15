fn sum_square_difference(limit: i32) -> i32 {
    let sum_of_square = (1..=limit).map(|x: i32| x.pow(2)).sum::<i32>();
    let square_of_sum = (1..=limit).sum::<i32>().pow(2);
    (square_of_sum - sum_of_square).abs()
}

#[cfg(test)]
mod tests {
    use crate::euler::sum_square_difference::sum_square_difference;

    #[test]
    fn test_1_to_10() {
        assert_eq!(sum_square_difference(10), 2640);
    }

    #[test]
    fn test_1_to_100() {
        assert_eq!(sum_square_difference(100), 25164150);
    }
}

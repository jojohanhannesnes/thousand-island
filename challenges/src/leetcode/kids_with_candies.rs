pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    candies
        .iter()
        .map(|num| *num + extra_candies >= *candies.iter().max().unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::kids_with_candies;
    #[test]
    fn it_works() {
        assert_eq!(
            kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        )
    }
}

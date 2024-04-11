use std::collections::HashSet;

// HashSet uses hash table, which makes it faster for lookups
// contains will check each value, but HashSet just need to hash and try to store in the same hash place
// if it's already stored, then its duplicate
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    for num in nums {
        if !seen.insert(num) {
            return true;
        }
    }
    false
}
#[cfg(test)]
mod tests {
    use super::contains_duplicate;
    #[test]
    fn it_works() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]))
    }
}

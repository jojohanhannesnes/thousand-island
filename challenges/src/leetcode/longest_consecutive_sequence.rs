use std::{cmp::max, collections::HashSet};

pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let mut longest = 0;
    let nums = nums.into_iter().collect::<HashSet<i32>>();
    for i in nums.iter() {
        let left = i - 1;
        let is_first = nums.contains(&left);
        if !is_first {
            let mut counter = 0;
            while nums.contains(&(i + counter)) {
                println!("{}", i + counter);
                counter += 1;
            }
            longest = max(longest, counter);
        }
    }
    longest
}

#[cfg(test)]
mod tests {
    use crate::leetcode::longest_consecutive_sequence::longest_consecutive;

    #[test]
    fn it_works() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }
}

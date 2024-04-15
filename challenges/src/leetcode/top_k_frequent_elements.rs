use std::collections::HashMap;

pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut stores = HashMap::new();
    for i in nums.into_iter() {
        stores.entry(i).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut stores = stores.into_iter().collect::<Vec<(i32, i32)>>();
    stores.sort_by(|a, b| b.1.cmp(&a.1));
    stores
        .iter()
        .map(|(k, _)| *k)
        .take(k as usize)
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::top_k_frequent_elements::top_k_frequent;

    #[test]
    fn it_works() {
        let nums = vec![1, 1, 1, 2, 2, 3];
        let mut result = top_k_frequent(nums, 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn empty() {
        let nums = vec![1, 2];
        let mut result = top_k_frequent(nums, 2);
        result.sort();
        assert_eq!(result, vec![1, 2]);
    }
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, x) in nums.iter().enumerate() {
        for (j, y) in nums.iter().enumerate() {
            if x + y == target && i != j {
                return Vec::from([i as i32, j as i32]);
            }
        }
    }
    panic!("input vec error")
}

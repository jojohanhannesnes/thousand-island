pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut position = Vec::new();
    let mut temp = 101;
    for (i, val) in nums.iter_mut().enumerate() {
        if temp == *val {
            position.push(i);
        } else {
            temp = *val;
        }
    }
    for i in position.iter().rev() {
        nums.remove(*i);
    }
    nums.len() as i32
}

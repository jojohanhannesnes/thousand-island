pub fn _x(nums: Vec<i32>) -> Vec<i32> {
    let mut res = Vec::new();
    for (index, _) in nums.iter().enumerate() {
        let right = &nums[index + 1..=nums.len() - 1];
        let left = &nums[0..index];
        let total = right.iter().product::<i32>() * left.iter().product::<i32>();
        println!("{total}");
        res.push(total);
    }
    res
}

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];
    for i in 1..nums.len() {
        result[i] = result[i - 1] * nums[i - 1];
    }
    let mut temp = 1;
    for i in (0..=nums.len() - 1).rev() {
        result[i] *= temp;
        temp *= nums[i];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::product_except_self;

    #[test]
    fn it_works() {
        let data = product_except_self(vec![1, 2, 3, 4]);
        assert_eq!(data, vec![24, 12, 8, 6])
    }

    #[test]
    fn second_test() {
        let data = product_except_self(vec![-1, 1, 0, -3, 3]);
        assert_eq!(data, vec![0, 0, 9, 0, 0])
    }
}

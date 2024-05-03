pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    let mut output = duration;
    if time_series.len() < 2 {
        return output;
    }
    for i in 0..=time_series.len() - 2 {
        let next = i + 1;
        let cur_poison_duration = time_series[i] + duration - 1;
        if cur_poison_duration >= time_series[next] {
            output += time_series[next] - time_series[i];
        } else {
            output += duration
        }
    }
    output
}

pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    nums1
        .into_iter()
        .map(|x| {
            let nums1_position = nums2.iter().position(|y| *y == x).unwrap();
            if let Some(found) = nums2[nums1_position..].iter().find(|q| q > &&x) {
                *found
            } else {
                -1
            }
        })
        .collect()
}

pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut found = false;
    for i in 0..=nums.len() - 1 {
        let mut next_pointer = if i == nums.len() - 1 { 0 } else { i + 1 };
        while next_pointer != i {
            if nums[next_pointer] > nums[i] {
                result.push(nums[next_pointer]);
                found = true;
                break;
            }
            next_pointer = if next_pointer + 1 > nums.len() - 1 {
                0
            } else {
                next_pointer + 1
            }
        }
        if !found {
            result.push(-1);
        }
    }
    result
}

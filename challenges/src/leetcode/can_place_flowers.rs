use std::collections::VecDeque;
pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    if n == 0 {
        return true;
    }
    let mut flowerbeds = VecDeque::from(flowerbed.clone());
    let mut counter = 0;
    flowerbeds.push_front(0);
    flowerbeds.push_back(0);
    for i in 1..flowerbed.len() + 1 {
        if flowerbeds[i] == 0 && flowerbeds[i + 1] == 0 && flowerbeds[i - 1] == 0 {
            flowerbeds[i] = 1;
            counter += 1;
        }
        if counter == n {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::leetcode::can_place_flowers::can_place_flowers;

    #[test]
    pub fn it_works() {
        assert!(can_place_flowers(vec![1, 0, 0, 0, 1], 1));
    }

    #[test]
    pub fn it_works_with_2() {
        assert!(can_place_flowers(vec![1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1], 2));
    }
}

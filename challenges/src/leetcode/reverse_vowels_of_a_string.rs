pub fn reverse_vowels(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let vowels: Vec<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        while left < right && !vowels.contains(&chars[left]) {
            left += 1;
        }
        while left < right && !vowels.contains(&chars[right]) {
            right -= 1;
        }
        if left < right {
            chars.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::leetcode::reverse_vowels_of_a_string::reverse_vowels;

    #[test]
    pub fn it_works() {
        assert_eq!(reverse_vowels(String::from("hello")), String::from("holle"));
    }
}

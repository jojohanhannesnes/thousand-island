pub fn str_str(haystack: String, needle: String) -> i32 {
    let vec_input: Vec<char> = haystack.chars().collect();
    for (i, val) in vec_input.windows(needle.len()).enumerate() {
        if needle.chars().collect::<Vec<char>>() == *val {
            return i as i32;
        }
    }
    -1
}

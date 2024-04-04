pub fn reverse_only_letters(s: String) -> String {
    let mut s = s.chars().collect::<Vec<char>>();
    let (mut start, mut end) = (0, s.len() - 1);
    while start < end {
        if !s[start].is_alphabetic() {
            start += 1;
        } else if !s[end].is_alphabetic() {
            end -= 1;
        } else {
            s.swap(start, end);
            start += 1;
            end -= 1;
        }
    }
    s.iter().collect()
}

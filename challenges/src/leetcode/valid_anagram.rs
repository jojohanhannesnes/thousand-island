use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    let mut first = HashMap::new();
    let mut second = first.clone();
    for i in s.chars() {
        let x = first.entry(i).or_insert(0);
        *x += 1;
    }

    for i in t.chars() {
        let x = second.entry(i).or_insert(0);
        *x += 1;
    }
    first == second
}
#[cfg(test)]
mod tests {
    use super::is_anagram;
    #[test]
    fn it_works() {
        assert!(is_anagram(String::from("anagram"), String::from("nagaram")))
    }
}

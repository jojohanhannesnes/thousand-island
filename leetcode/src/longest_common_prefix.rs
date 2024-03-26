pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let min_value = strs.iter().min_by_key(|x| x.len()).unwrap();
    let mut res = "";
    for i in 1..=min_value.len() {
        let prefixed = strs.iter().all(|x| x.starts_with(&min_value[0..i]));
        if prefixed {
            res = &min_value[0..i];
        } else {
            break;
        }
    }
    res.to_owned()
}

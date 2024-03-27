pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let candidate = strs.iter().min_by_key(|x| x.len()).unwrap();
    let mut res = "";
    for i in 1..=candidate.len() {
        let prefixed = strs.iter().all(|x| x.starts_with(&candidate[0..i]));
        if prefixed {
            res = &candidate[0..i];
        } else {
            break;
        }
    }
    res.to_owned()
}

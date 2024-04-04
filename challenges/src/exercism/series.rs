pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result: Vec<String> = Vec::with_capacity(len);
    if digits.len() < len {
        return vec![];
    }
    for i in 0..=(digits.len() - len) {
        println!("{}{}", i, len);
        let value: String = digits.chars().skip(i).take(len).collect();
        result.push(value);
        println!("value is : {:#?}", result);
    }
    result
}

#[test]
fn with_zero_length() {
    let expected = vec!["".to_string(); 6];
    assert_eq!(series("92017", 0), expected);
}
#[test]
fn with_length_2() {
    let expected = vec![
        "92".to_string(),
        "20".to_string(),
        "01".to_string(),
        "17".to_string(),
    ];
    assert_eq!(series("92017", 2), expected);
}
#[test]
fn with_numbers_length() {
    let expected = vec!["92017".to_string()];
    assert_eq!(series("92017", 5), expected);
}
#[test]
fn too_long() {
    let expected: Vec<String> = vec![];
    assert_eq!(series("92017", 6), expected);
}
#[test]
fn way_too_long() {
    let expected: Vec<String> = vec![];
    assert_eq!(series("92017", 42), expected);
}

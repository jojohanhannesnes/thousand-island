fn largest_palindrome_3_digit() -> Vec<(i32, i32, i32)> {
    let mut test = Vec::new();
    for i in 90..=99 {
        for y in i..=99 {
            let x = (i * y).to_string();
            let val: Vec<&str> = x.split("").collect();
            if val[0..val.len() / 2 - 1] == val[val.len() / 2 + 1..] {
                test.push((i, y, i * y));
            }
        }
    }
    test
}

#[test]
fn test() {
    assert_eq!(largest_palindrome_3_digit(), vec![(1, 2, 3)]);
}

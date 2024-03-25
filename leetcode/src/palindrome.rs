fn is_palindrome(x: i32) -> bool {
    let digits = x.to_string();
    let length = digits.len();
    println!(
        "len:{}, left:{}, right:{}",
        length,
        &digits[0..length / 2],
        &digits[length / 2..]
    );
    match length {
        0 => false,
        1 => true,
        _ if length % 2 == 0 => {
            digits[0..length / 2] == digits[length / 2..].chars().rev().collect::<String>()
        }
        _ if length % 2 == 1 => {
            digits[0..length / 2] == digits[length / 2 + 1..].chars().rev().collect::<String>()
        }
        _ => panic!("input undefined"),
    }
}

mod tests {
    use super::is_palindrome;
    #[test]
    fn is_palindromes() {
        assert!(is_palindrome(1001));
    }
}

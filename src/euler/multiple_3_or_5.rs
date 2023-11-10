// https://projecteuler.net/problem=1
// Use Modulus

fn sum_multiple_3_or_5(limit: i32) -> i32 {
    (1..limit)
        .filter(|x| (x % 3 == 0) || (x % 5 == 0))
        .sum::<i32>()
}

#[test]
fn test() {
    assert_eq!(sum_multiple_3_or_5(1000), 233168)
}

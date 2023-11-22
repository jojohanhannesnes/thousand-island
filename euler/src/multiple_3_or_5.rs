pub fn multiple_3_or_5(limit: i32) -> i32 {
    (1..limit)
        .filter(|x| (x % 3 == 0) || (x % 5 == 0))
        .sum::<i32>()
}

#[test]
fn it_works() {
    assert_eq!(multiple_3_or_5(1000), 233168);
}

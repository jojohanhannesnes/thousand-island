// // https://projecteuler.net/problem=5
// 1 2 3 4 5 6 7 8 9 10 =  2520 => 1 5 7 8 9
// 1..20 = ?? => 20 eliminate
// if x can be divided by 10, then it can be divided by 5, so we take 5, remove 10
// 2 4 6 8, if dividable by 8 then its dividable by 2, but we can't take 2,
// since it's possible that the number can have remainder if divided by 8
// example is 20

use std::collections::BTreeMap;

use crate::utils::list_of_prime_factor;

fn smallest_multiple() -> u32 {
    let mut db = BTreeMap::new();
    for i in 1..20 {
        let prime_factors = list_of_prime_factor(i);
        db.insert(i, prime_factors);
    }
    let mut max_occurrences: BTreeMap<i64, usize> = BTreeMap::new();

    for values in db.values() {
        for &num in values {
            let count = max_occurrences.entry(num).or_insert(0);
            *count = (*count).max(values.iter().filter(|&&x| x == num).count());
        }
    }

    let result = max_occurrences.iter().fold(1, |acc, (&num, &count)| {
        acc * (num as u32).pow(count as u32)
    });
    result
}

#[test]
fn test() {
    assert_eq!(smallest_multiple(), 232792560);
}

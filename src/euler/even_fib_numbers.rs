// https://projecteuler.net/problem=2

use crate::utils::fib::Fibonacci;

fn even_fib_numbers(limit: usize) -> usize {
    let mut y = Fibonacci::new(limit);
    y.even_fib();
    y.total
}

#[test]
fn test() {
    // 4613732
    assert_eq!(even_fib_numbers(4_000_000_usize), 4613732)
}

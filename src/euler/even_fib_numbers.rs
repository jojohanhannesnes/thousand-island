use crate::utils::fib::Fibonacci;

fn even_fib_numbers(limit: usize) -> usize {
    let mut y = Fibonacci::new(limit);
    y.even_fib();
    y.total
}

#[test]
fn test() {
    assert_eq!(even_fib_numbers(4_000_000_usize), 4613732)
}

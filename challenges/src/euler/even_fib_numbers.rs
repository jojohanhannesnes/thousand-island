use algorithms::fibonnaci::Fibonacci;

pub fn even_fib_numbers(limit: usize) -> usize {
    let mut fib = Fibonacci::new(limit);
    fib.even_fib();
    fib.total
}

#[test]
fn it_works() {
    assert_eq!(even_fib_numbers(4_000_000_usize), 4613732)
}

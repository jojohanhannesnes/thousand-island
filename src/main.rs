use utils::fib::Fibonacci;

mod utils;

fn main() {
    let mut f = Fibonacci::new(4000000_usize);
    let x = f.total;
    let y = &f.values;
    println!("{:?} = {x}", y);
    f.even_fib();
    println!("{:?} = {}", f.values, f.total);
}

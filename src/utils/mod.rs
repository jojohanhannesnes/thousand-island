pub mod fib;

pub fn list_of_prime_factor(mut input: i64) -> Vec<i64> {
    let mut start: i64 = 2; // first prime number
    let mut res = Vec::new();
    // sqrt(32) = 2 * 2 sqrt(8) -> 2,  2 * 2 sqrt(2) -> 2 * 2 , sqrt(2)
    while start.pow(2) <= input {
        if input % start == 0 {
            res.push(start);
            input /= start;
        } else {
            start += 1;
        }
    }
    if input > 1 {
        res.push(input); // last number inside the square root
    }

    res
}

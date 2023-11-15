use crate::utils::is_prime;

fn prime_10001() -> usize {
    let mut counter = 0;
    let mut result = 0;
    for i in 2..=usize::MAX {
        if counter == 10001 {
            break;
        }
        if is_prime(&i) {
            result = i;
            counter += 1;
        }
    }
    result
}

fn prime_10001_sieve() -> i32 {
    const LIMIT: usize = 150_000;
    let mut sieve = vec![true; LIMIT];
    let mut count = 0;

    for num in 2..LIMIT {
        if sieve[num] {
            count += 1;

            if count == 10_001 {
                return num as i32;
            }

            for multiple in (2 * num..LIMIT).step_by(num) {
                sieve[multiple] = false;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::euler::prime_10001::{is_prime, prime_10001, prime_10001_sieve};

    #[test]
    fn get_10001_prime() {
        let start_time = Instant::now();
        let result = prime_10001();
        let elapsed_time = start_time.elapsed();

        println!("Result: {}", result);
        println!("Time taken: {:?}", elapsed_time);
        assert_eq!(prime_10001(), 104743)
    }

    #[test]
    fn get_10001_prime_sieve() {
        let start_time = Instant::now();
        let result = prime_10001_sieve();
        let elapsed_time = start_time.elapsed();

        println!("Result: {}", result);
        println!("Time taken: {:?}", elapsed_time);
        assert_eq!(prime_10001(), 104743)
    }

    #[test]
    fn is_prime_13() {
        assert!(is_prime(&13))
    }
}

pub fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..((limit as f64).sqrt() as usize + 1) {
        if is_prime[i] {
            for j in (i * i..limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    (2..limit).filter(|&x| is_prime[x]).collect()
}

fn summation_of_primes(limit: usize) -> usize {
    sieve_of_eratosthenes(limit).iter().sum::<usize>()
}

#[test]
fn test() {
    use std::time::Instant;
    let start_time = Instant::now();
    let result = summation_of_primes(2_000_000);
    let elapsed_time = start_time.elapsed();

    println!("Result: {}", result);
    println!("Time taken: {:?}", elapsed_time);
    assert_eq!(summation_of_primes(2_000_000), 142913828922)
}

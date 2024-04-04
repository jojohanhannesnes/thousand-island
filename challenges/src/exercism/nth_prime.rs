pub fn nth(n: u32) -> u32 {
    let mut result = 0;
    let mut counter = 0;
    for i in 2..u32::MAX {
        let mut is_prime = true;
        let limit = (i as f64).sqrt() as u32;
        for j in 2..=limit {
            if i % j == 0 {
                is_prime = false; // Found a divisor, set is_prime to false
                break;
            }
        }
        if is_prime {
            counter += 1;
            result = i;
        }

        if counter == n + 1 {
            break;
        }
    }
    result
}

#[test]
fn first_prime() {
    assert_eq!(nth(0), 2);
}
#[test]
fn second_prime() {
    assert_eq!(nth(1), 3);
}
#[test]
fn sixth_prime() {
    assert_eq!(nth(5), 13);
}
#[test]
fn big_prime() {
    assert_eq!(nth(10_000), 104_743);
}

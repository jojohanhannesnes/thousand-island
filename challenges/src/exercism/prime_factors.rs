pub fn factors(q: u64) -> Vec<u64> {
    let mut num = q;
    let mut res = Vec::new();
    if num == 1 {
        return vec![];
    }
    let mut n: u64 = 2;
    while n.pow(2) <= num {
        if num % n == 0 {
            res.push(n);
            // num = (((num / n) as f32).floor()) as u64;
            num /= n;
            println!("{}", num);
        } else {
            n += 1;
        }
    }
    if num > 1 {
        res.push(num);
    }
    res
}

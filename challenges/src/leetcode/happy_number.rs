pub fn is_happy(n: i32) -> bool {
    let temp = format!("{}", n)
        .into_bytes()
        .into_iter()
        .map(|b| (b as i32 - 48).pow(2))
        .sum();
    if temp == 1 {
        return true;
    }
    let mut start = temp;
    loop {
        let total: i32 = format!("{}", start)
            .into_bytes()
            .into_iter()
            .map(|b| (b as i32 - 48).pow(2))
            .sum();
        if total == 1 {
            return true;
        } else if total == temp {
            return false;
        }
        start = total;
    }
}

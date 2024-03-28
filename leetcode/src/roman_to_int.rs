pub fn roman_to_int(s: String) -> i32 {
    let mut total = 0;
    let mut roman = s.chars().peekable();
    while let Some(c) = roman.next() {
        println!("{c}");
        let value = match c {
            'I' => {
                let next = roman.peek();
                if next == Some(&'V') {
                    roman.next();
                    4
                } else if next == Some(&'X') {
                    roman.next();
                    9
                } else {
                    1
                }
            }
            'V' => 5,
            'X' => {
                let next = roman.peek();
                if next == Some(&'L') {
                    roman.next();
                    40
                } else if next == Some(&'C') {
                    roman.next();
                    90
                } else {
                    10
                }
            }
            'L' => 50,
            'C' => {
                let next = roman.peek();
                if next == Some(&'D') {
                    roman.next();
                    400
                } else if next == Some(&'M') {
                    roman.next();
                    900
                } else {
                    100
                }
            }
            'D' => 500,
            'M' => 1000,
            _ => continue,
        };
        total += value;
    }
    total
}

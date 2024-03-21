fn roman_to_int(s: String) -> i32 {
    let mut c = s.chars();
    while let Some(s) = c.next() {
        // if char == 'b' {
        //     it.nth(1); // nth(1) skips/consumes exactly 2 items
        //     continue;
        // }
        // print!("{}", char);
        match s {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => continue,
        };
    }
    1
}

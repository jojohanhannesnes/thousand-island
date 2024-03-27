pub fn is_valid_hard(s: String) -> bool {
    let mut counter = HashMap::new();
    for c in s.chars() {
        match c {
            '{' | '(' | '[' => {
                counter.entry(c).and_modify(|x| *x += 1).or_insert(1);
            }
            '}' => {
                counter.entry('{').and_modify(|x| *x -= 1);
            }
            ']' => {
                counter.entry('[').and_modify(|x| *x -= 1);
            }
            ')' => {
                counter.entry('(').and_modify(|x| *x -= 1);
            }
            _ => todo!(),
        }
    }
    counter.values().all(|x| *x == 0)
}

pub fn is_valid_stack(s: String) -> bool {
    let mut counter = Vec::new();
    for c in s.chars() {
        match c {
            '{' | '(' | '[' => {
                counter.push(c);
            }
            '}' => {
                if !counter.is_empty() && *counter.last().unwrap() == '{' {
                    counter.pop();
                } else {
                    return false;
                }
            }
            ']' => {
                if !counter.is_empty() && *counter.last().unwrap() == '[' {
                    counter.pop();
                } else {
                    return false;
                }
            }
            ')' => {
                if !counter.is_empty() && *counter.last().unwrap() == '(' {
                    counter.pop();
                } else {
                    return false;
                }
            }
            _ => todo!(),
        }
    }
    counter.is_empty()
}

pub fn is_valid_easy(s: String) -> bool {
    let x: Vec<char> = s.chars().collect();
    for i in x.chunks(2) {
        if i != ['{', '}'] && i != ['(', ')'] && i != ['[', ']'] {
            return false;
        }
    }
    true
}

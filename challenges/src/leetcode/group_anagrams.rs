use std::collections::HashMap;

pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let sorted_str: String = chars.iter().collect();

        groups.entry(sorted_str).or_default().push(s);
    }

    groups.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sort_nested_vec(mut vec: Vec<Vec<String>>) -> Vec<Vec<String>> {
        vec.iter_mut().for_each(|inner_vec| inner_vec.sort());
        vec.sort();
        vec
    }

    #[test]
    fn it_works() {
        let result = group_anagrams(vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ]);

        let compares = vec![
            vec![String::from("bat")],
            vec![String::from("nat"), String::from("tan")],
            vec![
                String::from("ate"),
                String::from("eat"),
                String::from("tea"),
            ],
        ];

        assert_eq!(sort_nested_vec(result), sort_nested_vec(compares));
    }
}

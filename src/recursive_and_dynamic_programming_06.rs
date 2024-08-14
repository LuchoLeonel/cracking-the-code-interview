use std::collections::HashSet;

pub fn run() {
    let string_1 = "abc".to_string();
    let string_2 = "abcdefg".to_string();

    fn permutations(string: String) -> HashSet<String> {
        let mut set = HashSet::new();
        construct_permutation(&mut set, format!(""), string);
        set
    }

    fn construct_permutation(hashset: &mut HashSet<String>, prefix: String, remaining: String) {
        let len = remaining.len();
        if len == 0 {
            hashset.insert(prefix);
        } else {
            for i in 0..len {
                let mut new_remaining = remaining.clone();
                let new_letter = new_remaining.remove(i);
                let new_prefix = format!("{}{}", prefix, new_letter);
                construct_permutation(hashset, new_prefix, new_remaining);
            }
        }
    }

    let result_1 = permutations(string_1);
    dbg!(&result_1);
}
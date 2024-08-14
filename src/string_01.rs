use std::collections::HashMap;

pub fn run() {
    let string_1 = "abc";
    let string_2 = "bac";
    let string_3 = "";
    let string_4 = "hoasdajsda";
    let string_5 = "hola";
    let string_6 = "ohla";
    let string_7 = "";
    let string_8 = "jehg";
    let string_9 = "kdg";


    fn print(result: (&str, &str, bool)) {
        let (string1, string2, boolean) = result;
        print!("It's '{}' a permutation of '{}'?: {}\n", string1, string2, boolean);
    }

    fn is_permutation<'a>(string1: &'a str, string2: &'a str) -> (&'a str, &'a str, bool) {
        if string1.len() != string2.len() {
            return (string1, string2, false);
        }

        let mut chars_number1 = HashMap::new();
        let mut chars_number2 = HashMap::new();

        for ch in string1.chars() {
            *chars_number1.entry(ch).or_insert(1) += 1;
            match chars_number1.get(&ch) {
                Some(n) => chars_number1.insert(ch, n + 1),
                None => chars_number1.insert(ch, 1),
            };
        }

        for ch in string2.chars() {
            match chars_number2.get(&ch) {
                Some(n) => chars_number2.insert(ch, n + 1),
                None => chars_number2.insert(ch, 1),
            };
        }

        for ch in string1.chars() {
            let value1 = chars_number1.get(&ch).unwrap_or(&0);
            let value2 = chars_number2.get(&ch).unwrap_or(&0);
            if value1 != value2 {
                return (string1, string2, false);
            }
        }

        (string1, string2, true)
    }

    print(is_permutation(string_1, string_2));
    print(is_permutation(string_3, string_7));
    print(is_permutation(string_5, string_6));

    print(is_permutation(string_1, string_3));
    print(is_permutation(string_2, string_4));
    print(is_permutation(string_5, string_4));
    

    print(is_permutation(string_5, string_8));
    print(is_permutation(string_1, string_9));

    fn is_permutation_chat_gpt<'a>(string1: &'a str, string2: &'a str) -> (&'a str, &'a str, bool) {
        if string1.len() != string2.len() {
            return (string1, string2, false);
        }

        let mut chars_number = HashMap::new();

        for ch in string1.chars() {
            *chars_number.entry(ch).or_insert(0) += 1;
        }

        for ch in string2.chars() {
            *chars_number.entry(ch).or_insert(0) -= 1;
        }

        if chars_number.values().any(|&count| count != 0) {
            return (string1, string2, false);
        }

        (string1, string2, true)
    }
}
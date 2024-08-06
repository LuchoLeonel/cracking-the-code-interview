use std::collections::HashMap;

pub fn run() {
    let sentence1 = "Tact Coa";
    let sentence2 = "taco cat";
    let sentence3 = "atco eta";
    let sentence4 = "atco lta";
    let sentence5 = "atco ltagds";

    fn print(result: (&str, &str, bool)) {
        let (string1, string2, boolean) = result;
        print!("It's '{}' a palindrome permutation of '{}'?: {}\n", string1, string2, boolean);
    }

    fn is_palindrome_permutation<'a>(raw_string1: &'a str, raw_string2: &'a str) -> (&'a str, &'a str, bool) {
        let string1 = raw_string1.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase();
        let string2 = raw_string2.chars().filter(|c| !c.is_whitespace()).collect::<String>().to_lowercase();

        if string1.len() != string2.len() {
            return (raw_string1, raw_string2, false);
        }

        let mut counter = HashMap::new();

        for ch in string1.chars() {
            *counter.entry(ch).or_insert(0) +=1;
        }
        for ch in string2.chars() {
            *counter.entry(ch).or_insert(0) -=1;
        }

        for count in counter.values() {
            if *count != 0 {
                return (raw_string1, raw_string2, false);
            }
        }

        for ch in string1.chars() {
            *counter.entry(ch).or_insert(0) +=1;
        }

        let mut impar_counter = 0;
        for count in counter.values() {
            if *count %2 != 0 {
                impar_counter += 1;
                if impar_counter > 1 {
                    return (raw_string1, raw_string2, false);
                }
            }
        }

        (raw_string1, raw_string2, true)
    }

    print(is_palindrome_permutation(sentence1, sentence2));
    print(is_palindrome_permutation(sentence1, sentence3));
    print(is_palindrome_permutation(sentence2, sentence3));
    print(is_palindrome_permutation(sentence1, sentence4));
    print(is_palindrome_permutation(sentence1, sentence5));
    
}

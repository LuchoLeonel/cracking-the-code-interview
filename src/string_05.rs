pub fn run() {
    let sentence1 = "aabcccccaaa";
    let sentence2 = "abca";
    let sentence3 = "jjkkkksssdddd";
    let sentence4 = "ahhjskl";

    fn print(result: (&str, String)) {
        let (string1, string2) = result;
        print!("{} to: {}\n", string1, string2);
    }

    fn compression(string: &str) -> (&str, String) {
        let chars: Vec<char> = string.chars().collect();
        let mut sentence = String::new();

        for (index, ch) in chars.iter().enumerate() {
            if index != 0 && *ch == chars[index - 1] {
                continue;
            }
            let mut i = index;
            let mut counter = 0;
            while chars.len() > i && *ch == chars[i] {
                counter+=1;
                i+=1;
            }

            sentence.push(*ch);
            sentence.push_str(&counter.to_string());

            if string.len() < sentence.len()  {
                return (string, string.to_string());
            }
        }

        (string, sentence)
    }

    print(compression(sentence1));
    print(compression(sentence2));
    print(compression(sentence3));
    print(compression(sentence4));
}
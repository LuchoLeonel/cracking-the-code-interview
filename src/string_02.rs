
pub fn run() {
    let string1 = "Mr 3ohn Smith ";
    let string2 = "hola como estas";

    fn print(string: String) {
        print!("{}.\n", string);
    }

    fn urlify(string: &str) -> String {
        let chars: Vec<char> = string.chars().collect();
        let mut new_chars: Vec<char> = Vec::new();

        for (index, ch) in chars.iter().enumerate() {
            if *ch != ' ' {
                new_chars.push(*ch);
                continue;
            }

            if index == chars.len() - 1 {
                continue;
            }
            
            "%20".chars().for_each(|s| new_chars.push(s));
        }

        new_chars.iter().collect()
    }

    print(urlify(string1));
    print(urlify(string2));
    
    fn urlify_chat_gpt(string: &str) -> String {
        let mut new_string = String::new();

        for (index, ch) in string.chars().enumerate() {
            if ch != ' ' {
                new_string.push(ch);
            } else if index != string.len() - 1 {
                new_string.push_str("%20");
            }
        }

        new_string
    }

}
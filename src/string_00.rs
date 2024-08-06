pub fn run() {
    let string_1 = "hola";
    let string_2 = "hola como estas";
    let string_3 = "";
   
    fn is_unique(string: &str) -> (&str, bool) {
        for (index, c) in string.chars().enumerate() {
            for (index_ch, ch) in string.chars().enumerate() {
                if c == ch && index != index_ch {
                    return (string, false);
                }
            }
        }
        (string, true)
    }

    fn print(result: (&str, bool)) {
        let (string, boolean) = result;
        print!("It's '{}' unique?: {}\n", string, boolean);
    }

    print(is_unique(string_1));
    print(is_unique(string_2));
    print(is_unique(string_3));
}
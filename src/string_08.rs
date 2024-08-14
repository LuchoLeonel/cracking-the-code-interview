pub fn run() {
    let sentence1 = "rinoceronte";
    let sentence2 = "erinoceront";
    let sentence3 = "terinoceron";
    let sentence4 = "nterinocero";
    let sentence5 = "onterinocer";
    let sentence6 = "ronterinoce";
    let sentence7 = "eronterinoc";
    let sentence8 = "ceronterino";
    let sentence9 = "oceronterin";
    let sentence10 = "noceronteri";
    let sentence11 = "inoceronter";

    let sentence12 = "terinocsron";
    let sentence13 = "rinoceronteeee";
    let sentence14 = "holaaaaaaaa";

    fn print(result: (&str, &str, bool)) {
        let (string1, string2, boolean) = result;
        print!("It's '{}' a substring of '{}'?: {}\n", string1, string2, boolean);
    }

    fn is_substring<'a>(string1: &'a str, string2: &'a str) -> (&'a str, &'a str, bool) {
        if string1.len() != string2.len() {
            return (string1, string2, false);
        }

        if string1 == string2 {
            return (string1, string2, true);
        }

        let vec_char: Vec<char> = string2.chars().collect();
        let mut coincidente = false;
        let mut i = 1;
        while i < string1.len() {
            let mut new_string = String::new();
            let mut first = false;
            let mut j = i;
            while !first || j != i {
                first = true;
                if j >= string2.len() {
                    j = 0;
                }

                new_string.push(vec_char[j]);
                j+=1;
            }

            if new_string == string1 {
                coincidente = true;
                break;
            }
            i+=1;
        }

        (string1, string2, coincidente)
    }

    print(is_substring(sentence1, sentence2));
    print(is_substring(sentence1, sentence3));
    print(is_substring(sentence2, sentence3));
    print(is_substring(sentence1, sentence4));
    print(is_substring(sentence1, sentence5));
    print(is_substring(sentence1, sentence6));
    print(is_substring(sentence1, sentence7));
    print(is_substring(sentence1, sentence8));
    print(is_substring(sentence1, sentence9));
    print(is_substring(sentence1, sentence10));
    print(is_substring(sentence1, sentence11));
    print(is_substring(sentence1, sentence1));
    
    print(is_substring(sentence1, sentence12));
    print(is_substring(sentence1, sentence13));
    print(is_substring(sentence1, sentence14));

    fn is_substring_chatgpt<'a>(string1: &'a str, string2: &'a str) -> (&'a str, &'a str, bool) {
        if string1.len() != string2.len() {
            return (string1, string2, false);
        }
        
        let concatenated = format!("{}{}", string1, string1);
        let coincidente = concatenated.contains(string2);
        
        (string1, string2, coincidente)
    }
}
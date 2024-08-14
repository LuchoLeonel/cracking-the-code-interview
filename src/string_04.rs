
pub fn run() {
    let sentence1 = "hola como estas";
    let sentence2 = "hola como esta";
    let sentence3 = "hgla como estas";
    let sentence4 = "hola como estasg";
    let sentence5 = "hola comf estgs";
    let sentence6 = "hola como estasss";
    let sentence7 = "hola como estas, yo bien";
    let sentence8 = "hola como estakk";
    let sentence9 = "halo como ektas";

    fn print(result: (&str, &str, bool)) {
        let (string1, string2, boolean) = result;
        print!("It's '{}' one way of '{}'?: {}\n", string1, string2, boolean);
    }

    fn is_one_way<'a>(string1: &'a str, string2: &'a str) -> (&'a str, &'a str, bool) {
        let len_1 = string1.len() as isize;
        let len_2 = string2.len() as isize;
        if (len_1 - len_2).abs() > 1 {
            return (string1, string2, false);
        }
    
        let mut diff = 0;
        let mut coll1: Vec<char> = string1.chars().collect();
        let mut coll2: Vec<char> = string2.chars().collect();
    
        while !coll1.is_empty() && !coll2.is_empty() {
            if coll1[0] == coll2[0] {
                coll1.remove(0);
                coll2.remove(0);
            } else {
                diff += 1;
                if coll1.len() > 1 && coll1[1] == coll2[0] {
                    coll1.remove(0);
                } else if coll2.len() > 1 && coll1[0] == coll2[1] {
                    coll2.remove(0);
                } else {
                 if !coll1.is_empty() {
                        coll1.remove(0);
                    }
                    if !coll2.is_empty() {
                        coll2.remove(0);
                    }
                }
            }
        }
    
        if diff > 1 {
            return (string1, string2, false);
        }
    
        if !coll1.is_empty() || !coll2.is_empty() {
            diff += 1;
        }
    
        (string1, string2, diff <= 1)
    }

    print(is_one_way(sentence1, sentence2));
    print(is_one_way(sentence1, sentence3));
    print(is_one_way(sentence1, sentence4));
    print(is_one_way(sentence1, sentence5));
    print(is_one_way(sentence1, sentence6));
    print(is_one_way(sentence1, sentence7));
    print(is_one_way(sentence1, sentence8));
    print(is_one_way(sentence1, sentence9));

    fn is_one_way_chat_gpt<'a>(string1: &'a str, string2: &'a str) -> (&'a str, &'a str, bool) {
        let len_1 = string1.len();
        let len_2 = string2.len();
        if (len_1 as isize - len_2 as isize).abs() > 1 {
            return (string1, string2, false);
        }

       let (shorter, longer) = if len_1 > len_2 { (string1, string2) } else { (string2, string1) };
        let mut found_difference = false;
        let mut i = 0;
        let mut j = 0;

        while i < shorter.len() && j < longer.len() {
            if shorter.chars().nth(i) != longer.chars().nth(j) {
                if found_difference {
                    return (string1, string2, false);
                }
                found_difference = true;
                if len_1 == len_2 {
                    i += 1;
                }
            } else {
                i += 1;
            }
            j += 1;
        }

        (string1, string2, true)
    }
}
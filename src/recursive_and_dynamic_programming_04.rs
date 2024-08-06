pub fn run() {

    fn multiply(number_1: i32, number_2: i32) -> i32 {
        fn helper(small: i32, bigger: i32) -> i32 {
            if small == 1 {
                return bigger;
            }

            let s = small >> 1;
            let half_prod = helper(s, bigger);

            if small % 2 == 0 {
                half_prod + half_prod
            } else {
                half_prod + half_prod + bigger
            }
        }

        if number_1 == 0 || number_2 == 0 {
            return 0;
        }

        if number_1 < number_2 {
            helper(number_1, number_2)
        } else {
            helper(number_2, number_1)
        }
    }

    let number_1 = 5;
    let number_2 = 7;
    let result = multiply(number_1, number_2);
    print!("Multiply {} * {}: {}\n", number_1, number_2, result);
}
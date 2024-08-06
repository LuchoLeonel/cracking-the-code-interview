
pub fn run() {
    
    fn count_ways_to_clib(steps: i32) -> i32 {  
         count_steps(steps)
    }

    fn count_steps(steps: i32) -> i32 {
        if steps == 0 || steps == 1 {
            return 1;
        } else if steps == 2 {
            return 2;
        }

        return count_steps(steps - 1) + count_steps(steps - 2) + count_steps(steps - 3);
    }


    let count = count_ways_to_clib(10);
    print!("Ways to climb the staircase: {}\n", count);
}
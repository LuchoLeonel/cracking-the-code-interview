pub fn run() {

    fn magic_index_for_sorted_array(sorted_array: &Vec<i32>, start: usize, end: usize) -> Option<i32> {
        if start > end {
            return None;
        }

        let middle_index = start + ((end - start) / 2);
        let middle_value = sorted_array[middle_index];

        if middle_value == middle_index as i32 {
            return Some(middle_value);
        }

        if middle_value < middle_index as i32 {
            magic_index_for_sorted_array(sorted_array, middle_index+1, end)
        } else {
            magic_index_for_sorted_array(sorted_array, start, middle_index-1)
        }
    }

    let sorted_array = vec![-10, -3, -1, 1, 2, 5, 7, 9, 10];
    
    let magic_index = magic_index_for_sorted_array(&sorted_array, 0, sorted_array.len() - 1).unwrap();
    print!("Magic index: {}\n", magic_index);
}
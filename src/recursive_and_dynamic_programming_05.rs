pub fn run() {

    let mut tower_a: Vec<i32> = vec![5, 4, 3, 2, 1];
    let mut tower_b: Vec<i32> = Vec::new();
    let mut tower_c: Vec<i32> = Vec::new();

    fn move_from_a_to_c(tower_a: &mut Vec<i32>, tower_b: &mut Vec<i32>, tower_c: &mut Vec<i32>) {
        let len = tower_a.len() as i32;  
        move_disk_from_to(len, tower_a, tower_c, tower_b);
    }

    fn move_disk_from_to(n: i32, from: &mut Vec<i32>, to: &mut Vec<i32>, auxiliary: &mut Vec<i32>) {
        if n == 0 {
            return;
        }

        move_disk_from_to(n-1, from, auxiliary, to);
        if let Some(disk) = from.pop() {
            to.push(disk);
        }
        move_disk_from_to(n-1, auxiliary, to, from);
    }


    move_from_a_to_c(&mut tower_a, &mut tower_b, &mut tower_c);

    dbg!(&tower_a);
    dbg!(&tower_c);
    dbg!(&tower_b);
}
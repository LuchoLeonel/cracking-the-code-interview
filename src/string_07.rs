pub fn run() {
    
    fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        if matrix.is_empty() || matrix[0].is_empty() {
            return;
        }

        let rows = matrix.len();
        let columns = matrix[0].len();
        let mut rows_to_zero = vec![false; rows];
        let mut columns_to_zero = vec![false; columns];

        for i in 0..rows {
            for j in 0..columns {
                if matrix[i][j] == 0 {
                    rows_to_zero[i] = true;
                    columns_to_zero[j] = true;
                }
            }
        }


        for j in 0..columns {
            if columns_to_zero[j] {
                for i in 0..rows {
                    matrix[i][j] = 0;
                }
            }
        }

        for i in 0..rows {
            if rows_to_zero[i] {
                for j in 0..columns {
                    matrix[i][j] = 0;
                }
            }
        }
    }

    let mut matrix = vec![
        vec![1, 2, 3, 4],
        vec![5, 0, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 0],
    ];

    set_zeroes(&mut matrix);

    for row in matrix {
        println!("{:?}", row);
    }
}
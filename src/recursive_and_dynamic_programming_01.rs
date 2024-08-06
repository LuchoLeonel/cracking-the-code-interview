use std::collections::HashSet;

pub fn run() {
    type Node = (usize, usize);

    #[derive(Debug, PartialEq)]
    enum Direction {
        Right,
        Down,
    }

    #[derive(Debug)]
    struct Grid {
        grid: Vec<Vec<bool>>,
    }

    impl Grid {
        fn new(values: Vec<Vec<bool>>) -> Grid {
            Grid { grid: values }
        }

        fn find_path(&self) -> Option<Vec<Direction>> {
            let mut path: Vec<Direction> = Vec::new();
            let mut visited: HashSet<Node> = HashSet::new();
            if self.depth_first_search(0, 0, &mut path, &mut visited) {
                Some(path)
            } else {
                None
            }
        }

        fn depth_first_search(&self, row: usize, col: usize, path: &mut Vec<Direction>, visited: &mut HashSet<Node>) -> bool {
            if row >= self.grid.len() || col >= self.grid[0].len() || !self.grid[row][col] {
                return false;
            }

            if row == self.grid.len() - 1 && col == self.grid[0].len() - 1 {
                return true;
            }

            let current_node = (row, col);
            if visited.contains(&current_node) {
                return false;
            }

            visited.insert(current_node);

            if self.depth_first_search(row, col + 1, path, visited) {
                path.push(Direction::Right);
                return true;
            }

            if self.depth_first_search(row + 1, col, path, visited) {
                path.push(Direction::Down);
                return true;
            }

            visited.remove(&current_node);
            false
        }
    }

    let raw_grid = vec![
        vec![true, false, true, true],
        vec![true, true, false, true],
        vec![false, true, false, true],
        vec![true, true, true, true]
    ];

    let grid = Grid::new(raw_grid);
    let path = grid.find_path();
    dbg!(&path);
}

fn main() {
    run();
}
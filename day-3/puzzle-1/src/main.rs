use std::fs;

struct Map {
    matrix: Vec<Vec<bool>>,
}

impl Map {
    fn new(input: &str) -> Map{
        let lines: Vec<&str> = input.split('\n').collect();
        let matrix: Vec<Vec<bool>> = lines.iter().map(|line|{
            let chars = line.chars();
            let tree_indicators: Vec<bool> = chars.map(|c| c == '#').collect();
            tree_indicators
        }).collect();
        Map {
            matrix
        }
    }

    fn index_by_row_and_column(&self, row: usize, col: usize) -> bool{
        let row: Vec<bool> = self.matrix[row].clone();
        let col_to_index: usize = col % row.len();
        row[col_to_index]
    }

    fn row_length(&self) -> usize{
        self.matrix.len()
    }
}

fn main() {
    let mut contents = fs::read_to_string("../input.txt")
        .expect("Something went wrong!");
    contents = contents.trim().to_string();
    let map = Map::new(&contents);
    let row_length = map.row_length();

    let mut current_row = 0;
    let mut current_col = 0;
    let mut trees_encountered = 0;

    while current_row < row_length {
        if map.index_by_row_and_column(current_row, current_col) {
            trees_encountered = 1 + trees_encountered;
        }
        current_row += 1;
        current_col += 3;
    }

    println!("Trees encountered: {}", trees_encountered);
}

use std::fs;

#[derive(Clone)]
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

fn simulate_slope(map: Map, right_amount: usize, down_amount: usize) -> i64 {
    let row_length = map.row_length();
    let mut current_row = 0; 
    let mut current_col = 0;
    let mut trees_encountered = 0;

    while current_row < row_length {
        if map.index_by_row_and_column(current_row, current_col) {
            trees_encountered += 1;
        }

        current_row += down_amount;
        current_col += right_amount;
    }

    trees_encountered
}

fn main() {
    let mut contents = fs::read_to_string("../input.txt")
        .expect("Something went wrong!");
    contents = contents.trim().to_string();
    let map = Map::new(&contents);

    let right_1_down_1 = simulate_slope(map.clone(), 1, 1);
    let right_3_down_1 = simulate_slope(map.clone(), 3, 1);
    let right_5_down_1 = simulate_slope(map.clone(), 5, 1);
    let right_7_down_1 = simulate_slope(map.clone(), 7, 1);
    let right_1_down_2 = simulate_slope(map, 1, 2);

    let trees_encountered = right_1_down_1 * right_3_down_1 * right_5_down_1 * right_7_down_1 * right_1_down_2;

    println!("Trees encountered: {}", trees_encountered);
}
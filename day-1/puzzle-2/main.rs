use std::fs;
use std::collections::HashMap;

fn main(){
    let contents = fs::read_to_string("../input.txt")
        .expect("Something went wrong!");
    
    let numbers: Vec<i64> = contents.split('\n').filter_map(|s| s.parse::<i64>().ok()).collect();
    let mut diff_to_nums: HashMap<i64, (i64,i64)> = HashMap::new();

    for first_number in numbers.clone() {
        for second_number in numbers.clone() {
            if first_number != second_number {
                let diff = 2020 - first_number - second_number;
                diff_to_nums.insert(
                    diff,
                    (first_number.clone(), second_number.clone()),
                );
            }
        }
    }

    for number in numbers {
        match diff_to_nums.get(&number){
            Some((second_num,third_num)) => {
                let product = number * *second_num * *third_num;
                println!("{} is the answer", product);
            }, 
            None => continue,
        }
    }
}
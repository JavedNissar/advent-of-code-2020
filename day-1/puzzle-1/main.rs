use std::fs;
use std::collections::HashMap;

fn main(){
    let contents = fs::read_to_string("../input.txt")
        .expect("Something went wrong!");
    
    let numbers: Vec<i64> = contents.split('\n').filter_map(|s| s.parse::<i64>().ok()).collect();
    let mut diff_to_num: HashMap<i64, i64> = HashMap::new();

    for number in numbers.clone() {
        diff_to_num.insert(
            2020 - number,
            number.clone(),
        );
    }

    for number in numbers {
        match diff_to_num.get(&number){
            Some(other_num) => {
                let product = number * *other_num;
                println!("{} is the answer", product);
            }, 
            None => continue,
        }
    }
}
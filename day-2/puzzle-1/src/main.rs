use std::fs;

struct PasswordPolicy {
    minimum_number_of_times: i32,
    maximum_number_of_times: i32,
    character: char,
}

impl PasswordPolicy{
    fn new(input: &str) -> PasswordPolicy {
        let components: Vec<&str> = input.split(' ').collect();
        let numbers: Vec<i32> = components[0].split('-').filter_map(|s| s.parse::<i32>().ok()).collect();
        let char_to_check = components[1].chars().nth(0).unwrap();
        let min: i32 = numbers[0];
        let max: i32 = numbers[1];
        return PasswordPolicy{
            minimum_number_of_times: min,
            maximum_number_of_times: max,
            character: char_to_check,
        };
    }

    fn validate(&self, to_check: &str) -> bool {
        let mut count = 0;
        for character in to_check.chars() {
            if self.character == character {
                count += 1;
            }
        }

        let greater_than_min = self.minimum_number_of_times <= count;
        let less_than_max = count <= self.maximum_number_of_times;
        return greater_than_min && less_than_max;
    }
}

fn parse(line: &str) -> i32 {
    let components: Vec<&str> = line.split(':').collect();
    let policy = PasswordPolicy::new(components[0]);

    let password_to_check = components[1];

    if policy.validate(password_to_check){
        1
    }else{
        0
    }
}

fn main(){
    let contents = fs::read_to_string("../input.txt")
        .expect("Something went wrong!");
    let lines: Vec<&str> = contents.split('\n').collect();

    let mut count = 0;
    for line in lines{
        if !line.is_empty(){
            count += parse(line);
        }
    }

    println!("The number of valid passwords:{}", count);
}

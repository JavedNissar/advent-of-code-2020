use std::fs;

struct PasswordPolicy {
    first_position: usize,
    second_position: usize,
    character: char,
}

impl PasswordPolicy{
    fn new(input: &str) -> PasswordPolicy {
        let components: Vec<&str> = input.split(' ').collect();
        let numbers: Vec<usize> = components[0].split('-').filter_map(|s| s.parse::<usize>().ok()).collect();
        let char_to_check = components[1].chars().nth(0).unwrap();
        let first_position: usize = numbers[0];
        let second_position: usize = numbers[1];
        return PasswordPolicy{
            first_position: first_position,
            second_position: second_position,
            character: char_to_check,
        };
    }

    fn validate(&self, to_check: &str) -> bool {
        let to_check_chars: Vec<char> = to_check.trim().chars().collect();

        let first_position_char = to_check_chars[self.first_position - 1];
        let second_position_char = to_check_chars[self.second_position - 1];

        let does_first_position_contain = first_position_char == self.character;
        let does_second_position_contain = second_position_char == self.character;

        (does_first_position_contain || does_second_position_contain) && does_first_position_contain != does_second_position_contain
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
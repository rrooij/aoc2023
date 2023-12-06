use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let splitted = contents.lines();
    let mut numbers = Vec::new();
    for line in splitted {
        let mut first_number = 0;
        let mut last_number = 0;
        for character in line.chars() {
            if let Some(number) = character.to_digit(10) {
                if first_number == 0 {
                    first_number = number;
                }
                last_number = number;
            }
        }
        numbers.push(format!("{}{}", first_number, last_number).parse::<u32>().unwrap());
    }
    println!("{}", numbers.iter().sum::<u32>());
}

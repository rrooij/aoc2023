use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let splitted = contents.lines();
    let mut numbers = Vec::new();
    let word_mapping = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4"),
        ("five", "f5e"),
        ("six", "s6"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "9e")
    ]);
    let digit_words: Vec<&str> = word_mapping.clone().into_keys().collect();
    for line in splitted {
        let mut clean_line = String::from(line);
        for digit in &digit_words {
            clean_line = clean_line.replace(digit, word_mapping[digit]);
        }
        let mut first_number = 0;
        let mut last_number = 0;
        for character in clean_line.chars() {
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

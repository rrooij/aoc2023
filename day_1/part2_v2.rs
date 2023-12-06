use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let splitted = contents.lines();
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
    let numbers = splitted.map(|line| {
        let mut clean_line = String::from(line);
        for digit in &digit_words {
            clean_line = clean_line.replace(digit, word_mapping[digit]);
        }
        let numbers: Vec<char> = clean_line.chars().filter(|character | character.to_digit(10).is_some()).collect();
        let last_idx = numbers.len() - 1;
        format!("{}{}", numbers[0], numbers[last_idx]).parse::<u32>().unwrap()
    });
    println!("{}", numbers.sum::<u32>());
}

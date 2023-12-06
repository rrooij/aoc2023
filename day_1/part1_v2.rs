use std::fs;

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let splitted = contents.lines();
    let numbers = splitted.map(|line| {
        let numbers: Vec<char> = line.chars().filter(|character | character.is_ascii_digit()).collect();
        let last_idx = numbers.len() - 1;
        format!("{}{}", numbers[0], numbers[last_idx]).parse::<u32>().unwrap()
    });
    println!("{}", numbers.sum::<u32>());
}

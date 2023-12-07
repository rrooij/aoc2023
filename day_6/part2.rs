use std::fs;

#[derive(Clone, Debug)]
struct Race {
    time: u64,
    distance: u64,
}

const BOAT_START_SPEED_PRESSED_MS: u64 = 0;

impl Race {
    fn possibilities(&self) -> u64 {
        let mut possibilities = 0;
        for i in 1..self.time {
            let boat_speed = BOAT_START_SPEED_PRESSED_MS + i;
            let remaining_time = i..self.time;
            let mut distance = 0;
            for _ in remaining_time {
                distance += boat_speed
            }
            if distance > self.distance {
                possibilities += 1;
            }
        }
        possibilities
    }
}

fn get_numbers(line: &str) -> u64 {
    line.split(' ')
        .filter(|x| x.parse::<u64>().is_ok())
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap()
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let time = get_numbers(lines[0]);
    let distance = get_numbers(lines[1]);
    let race = Race { time, distance };
    println!("{}", race.possibilities());
}

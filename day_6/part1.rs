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

fn get_numbers(line: &str) -> Vec<u64> {
    line.split(' ')
        .filter_map(|x| x.parse::<u64>().ok())
        .collect()
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let times = get_numbers(lines[0]);
    let distances = get_numbers(lines[1]);
    let races: Vec<Race> = times
        .iter()
        .zip(distances.iter())
        .map(|(time, distance)| Race {
            time: *time,
            distance: *distance,
        })
        .collect();
    let possibilities_product = races.iter().fold(1, |acc, race| race.possibilities() * acc);
    println!("{}", possibilities_product);
}

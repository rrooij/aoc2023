use std::fs;

#[derive(Debug)]
struct SeedMap {
    name: String,
    seeds: Vec<u64>,
    targets: Vec<u64>
}

fn find_lowest(taken_seeds: Vec<u64>, maps: Vec<SeedMap>) {
    let result: u64 = taken_seeds.iter().map(|seed| {
        let mut result = seed.clone();
        for map in &maps {
            result = match map.seeds.iter().position(|c| *c == result) {
                Some(c) => map.targets[c],
                None => result
            };
        }
        result
    }).min().unwrap();
    println!("The minimum is {}", result)
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let splitted: Vec<&str> = contents.lines().collect();
    let seeds: Vec<u64> = splitted[0].split(' ')
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect();
    let maps: Vec<SeedMap> = contents.split("\n\n").skip(1)
        .map(|raw_map| {
            let splitted_colon_map: Vec<&str> = raw_map.split(':').collect();
            let name = splitted_colon_map[0];
            let numbers = splitted_colon_map[1]
                .split("\n")
                .filter(|x| x != &"")
                .map(|x| {
                    let number_line: Vec<u64> = x.split(' ').map(|y| y.parse::<u64>().unwrap()).collect();
                    let range = number_line[2];
                    println!("Seed range from {} to {}", number_line[0], number_line[0] + range);
                    let seed_range: Vec<u64> = (number_line[0]..number_line[0] + range).collect();
                    let target_range: Vec<u64> = (number_line[1]..number_line[1] + range).collect();
                    println!("Target range from {} to {}", number_line[1], number_line[1] + range);
                    vec![seed_range, target_range]
                })
                .fold(vec![vec![], vec![]], |acc: Vec<Vec<u64>>, x| {
//                    vec![vec![], vec![]]
                    vec![vec![acc[0].clone(), x[0].clone()].concat(), vec![acc[1].clone(), x[1].clone()].concat()]
                });
            SeedMap { name: name.to_string(), targets: numbers[0].clone(), seeds: numbers[1].clone() }
        })
        .collect();
    find_lowest(seeds, maps);
}

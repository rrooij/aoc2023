use rayon::prelude::*;
use std::fs;
use std::ops::Range;

#[derive(Debug)]
struct SeedMap {
    mapping: Vec<(Range<u64>, Range<u64>)>,
}

fn find_lowest(taken_seeds: Vec<u64>, maps: Vec<SeedMap>) {
    let result: u64 = taken_seeds
        .par_iter()
        .map(|seed| {
            let mut result = *seed;
            for map in &maps {
                for (target, source) in &map.mapping {
                    let start = source.start;
                    let end = source.end;
                    if result < start || result >= end {
                        continue;
                    }
                    result = target.start + (result - start);
                    break;
                }
            }
            result
        })
        .min()
        .unwrap();
    println!("The minimum is {}", result)
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let splitted: Vec<&str> = contents.lines().collect();
    let seeds: Vec<u64> = splitted[0]
        .split(' ')
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .flat_map(|x| (x[0]..x[0] + x[1]))
        .collect();
    let maps: Vec<SeedMap> = contents
        .split("\n\n")
        .skip(1)
        .map(|raw_map| {
            let splitted_colon_map: Vec<&str> = raw_map.split(':').collect();
            let numbers: Vec<(Range<u64>, Range<u64>)> = splitted_colon_map[1]
                .split('\n')
                .filter(|x| x != &"")
                .map(|x| {
                    let number_line: Vec<u64> =
                        x.split(' ').map(|y| y.parse::<u64>().unwrap()).collect();
                    let range = number_line[2];
                    let seed_range: Range<u64> = number_line[1]..number_line[1] + range;
                    let target_range: Range<u64> = number_line[0]..number_line[0] + range;
                    (target_range, seed_range)
                })
                .collect();
            SeedMap { mapping: numbers }
        })
        .collect();
    find_lowest(seeds, maps);
}

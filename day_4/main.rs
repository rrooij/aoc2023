use std::fs;

#[derive(Clone, Debug)]
struct Card {
    winning: Vec<u32>,
    takes: Vec<u32>,
    copies: u32,
}

impl Card {
    fn winning_takes(&self) -> Vec<u32> {
        self.takes.clone().into_iter().filter(|x| self.winning.contains(x)).collect()
    }

    fn count_points(&self) -> u32 {
        let mut score = 0;
        for (i, _x) in self.winning_takes().iter().enumerate() {
            match i {
                0 => score = 1,
                _ => score *= 2
            }
        }
        score
    }
}

fn count_scratchcards(cards: Vec<Card>) {
    let mut new_cards = cards.clone();
    let length = cards.len();
    for idx in 0..length {
        let game_id = idx + 1;
        let winning = new_cards[idx].winning_takes().len();
        for _x in 0..new_cards[idx].copies {
            for idx2 in game_id..game_id + winning {
                new_cards[idx2].copies += 1;
            }
        }
    }
    let total_count: u32 = new_cards.clone().into_iter().map(|x| x.copies).sum();
    println!("Total count: {}", total_count)
}

fn main() {
    let contents = fs::read_to_string("./input.txt").unwrap();
    let splitted = contents.lines();
    let cards: Vec<Card> = splitted.map( |line| {
        let cleaned_line = line.replace(':', "");
        let split_sep: Vec<&str> = cleaned_line.split('|').collect();
        let space_split_left: Vec<&str> = split_sep[0].split(' ').collect();
        let left_numbers: Vec<u32> = space_split_left.into_iter().filter_map(|x| x.parse::<u32>().ok()).collect();
        let winning: Vec<u32> = left_numbers[1..].to_vec();
        let space_split_right: Vec<&str> = split_sep[1].split(' ').collect();
        let takes: Vec<u32> = space_split_right.into_iter().filter_map(|x| x.parse::<u32>().ok()).collect();
        Card { winning, takes, copies: 1 }
    }).collect();
    let score: u32 = cards.iter().map(|x| x.count_points()).sum();
    println!("Score: {}", score);
    count_scratchcards(cards);
}

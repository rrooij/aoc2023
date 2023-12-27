use std::collections::HashMap;
use std::fs;

#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u64,
    hand_type: Score,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum Score {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn default(cards: Vec<char>, bid: u64) -> Hand {
        Hand {
            cards: cards.clone(),
            bid,
            hand_type: Hand::get_score(cards),
        }
    }

    fn get_score(cards: Vec<char>) -> Score {
        let mut map: HashMap<char, u64> = HashMap::new();
        for card in cards {
            match map.get(&card) {
                Some(amount) => {
                    let new_amount = amount + 1;
                    map.insert(card, new_amount);
                }
                _ => {
                    map.insert(card, 1);
                }
            };
        }
        let mut map_values: Vec<u64> = map.into_values().collect();
        map_values.sort_by(|a, b| b.cmp(a));
        if map_values[0] == 5 {
            return Score::FiveOfAKind;
        } else if map_values[0] == 4 {
            return Score::FourOfAKind;
        } else if map_values[0] == 3 && map_values[1] == 2 {
            return Score::FullHouse;
        } else if map_values[0] == 3 {
            return Score::ThreeOfAKind;
        } else if map_values[0] == 2 && map_values[1] == 2 {
            return Score::TwoPair;
        } else if map_values[0] == 2 {
            return Score::OnePair;
        }
        Score::HighCard
    }
}

fn main() {
    let card_rank: HashMap<char, u64> = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]);
    let contents = fs::read_to_string("./input.txt").unwrap();
    let splitted = contents.lines();
    let mut hands: Vec<Hand> = splitted
        .map(|line| {
            let split_line: Vec<&str> = line.split(' ').collect();
            Hand::default(
                split_line[0].chars().collect(),
                split_line[1].parse::<u64>().unwrap(),
            )
        })
        .collect();
    dbg!(hands.clone());
    hands.sort_by(|a, b| {
        if a.hand_type == b.hand_type {
            for i in 0..5 {
                let card_a = a.cards[i];
                let card_b = b.cards[i];
                if card_a == card_b {
                    continue;
                }
                return card_rank
                    .get(&a.cards[i])
                    .unwrap()
                    .partial_cmp(card_rank.get(&b.cards[i]).unwrap())
                    .unwrap();
            }
        }
        b.hand_type.partial_cmp(&a.hand_type).unwrap()
    });
    let mut sum: u64 = 0;
    for (i, hand) in hands.iter().enumerate() {
        let rank = i as u64 + 1;
        sum += hand.bid * rank;
    }
    println!("The sum is {}", sum);
}

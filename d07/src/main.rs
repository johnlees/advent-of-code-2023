use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum HandType {                     // w/ jokers
    FiveOfAKind,  // (5)            // (FiveOfAKind)
    FourOfAKind,  // (4,1)          // (FiveOfAKind, FiveOfAKind)
    FullHouse,    // (3,2)          // (FiveOfAKind, FiveOfAKind)
    ThreeOfAKind, // (3,1,1)        // (FourOfAKind, FourOfAKind, FourOfAKind)
    TwoPair,      // (2,2,1)        // (FourOfAKind, FourOfAKind, FullHouse)
    OnePair,      // (2,1,1,1)      // (ThreeOfAKind,...)
    HighCard,     // (1,1,1,1,1)    // (OnePair,...)
}

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    rank: HandType,
    bid: usize,
}

impl Hand {
    fn new(line: &str, jokers: bool) -> Self {
        let fields: Vec<&str> = line.split_whitespace().collect();
        let cards = fields[0]
            .chars()
            .map(|c| match c {
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                'T' => 10,
                'J' => if jokers {1} else {11},
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => 0,
            })
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();
        let bid = usize::from_str_radix(fields[1], 10).unwrap();

        let mut map: HashMap<u8, usize> = HashMap::new();
        for c in cards {
            *map.entry(c).or_default() += 1;
        }
        let max_cnt = *map.values().max().unwrap();
        let mut rank = match map.len() {
            1 => HandType::FiveOfAKind,
            2 => match max_cnt {
                4 => HandType::FourOfAKind,
                3 => HandType::FullHouse,
                _ => HandType::HighCard,
            },
            3 => match max_cnt {
                3 => HandType::ThreeOfAKind,
                2 => HandType::TwoPair,
                _ => HandType::HighCard,
            },
            4 => HandType::OnePair,
            _ => HandType::HighCard,
        };
        if jokers && map.contains_key(&1) {
            rank = match rank {
                HandType::FiveOfAKind => HandType::FiveOfAKind,
                HandType::FourOfAKind => HandType::FiveOfAKind,
                HandType::FullHouse => HandType::FiveOfAKind,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::TwoPair => if map[&1] == 1 {HandType::FullHouse} else {HandType::FourOfAKind},
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::HighCard => HandType::OnePair,
            }
        }

        Self { cards, rank, bid }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut compare = self.rank.cmp(&other.rank);
        if compare == Ordering::Equal {
            for (c1, c2) in self.cards.iter().zip(&other.cards) {
                if c1 > c2 {
                    compare = Ordering::Less;
                    break;
                } else if c2 > c1 {
                    compare = Ordering::Greater;
                    break;
                }
            }
        }
        compare
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        (self.cards, &self.rank) == (other.cards, &other.rank)
    }
}

impl Eq for Hand {}

fn main() {
    // Parse input
    let hands = read_hands("input.txt", false);
    println!("Part 1: {}", calc_winnings(&hands));
    let joker_hands = read_hands("input.txt", true);
    println!("Part 2: {}", calc_winnings(&joker_hands));
}

fn calc_winnings(hands: &[Hand]) -> usize {
    let mut winnings = 0;
    for (rank, hand) in hands.iter().enumerate() {
        //println!("{hand:?}");
        winnings += (hands.len() - rank) * hand.bid;
    }
    winnings
}

fn read_hands(filename: &str, jokers: bool) -> Vec<Hand> {
    let mut hands = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if let Ok(ip) = line {
                hands.push(Hand::new(&ip, jokers));
            }
        }
    }
    hands.sort();
    hands
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

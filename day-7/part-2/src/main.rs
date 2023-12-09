use countmap::CountMap;
use itertools::sorted;
use std::cmp::Ordering;
use std::fs;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
struct Card {
    symbol: char,
}

impl Card {
    const SYMBOLS: [char; 13] = [
        'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
    ];

    fn from_char(c: char) -> Option<Self> {
        if Self::SYMBOLS.contains(&c) {
            Some(Card { symbol: c })
        } else {
            None
        }
    }

    fn strength(self: &Self) -> i32 {
        Self::SYMBOLS
            .iter()
            .position(|c| c == &self.symbol)
            .unwrap() as i32
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Hand {
    cards: Vec<Card>,
    groups: Vec<i32>,
}

impl Hand {
    fn from_string(s: &str) -> Option<Self> {
        let cards_opt: Vec<Option<Card>> = s.chars().map(|c| Card::from_char(c)).collect();
        if cards_opt.iter().all(|c| c.is_some()) {
            let cards: Vec<Card> = cards_opt.into_iter().flatten().collect();
            let mut group_counter: CountMap<char, i32> = CountMap::new();
            let mut jokers = 0;
            for card in cards.iter() {
                if card.symbol == 'J' {
                    jokers += 1;
                } else {
                    group_counter.insert_or_increment(card.symbol);
                }
            }
            let mut groups: Vec<i32> = sorted(group_counter.values()).rev().cloned().collect();
            if groups.is_empty() {
                groups.push(jokers);
            } else {
                groups[0] += jokers;
            }
            Some(Hand { cards, groups })
        } else {
            None
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let groups_ord = self.groups.cmp(&other.groups);
        let self_strength_iter = self.cards.iter().map(|c| c.strength());
        let other_strength_iter = other.cards.iter().map(|c| c.strength());
        let strength_ord = self_strength_iter.cmp(other_strength_iter);
        if groups_ord == Ordering::Equal {
            strength_ord
        } else {
            groups_ord
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct Bid {
    hand: Hand,
    value: i64,
}

impl Bid {
    fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        match &parts[..] {
            &[hand_str, value_str] => {
                let hand = Hand::from_string(hand_str);
                let value = value_str.parse::<i64>().ok();
                match (hand, value) {
                    (Some(hand), Some(value)) => Some(Bid { hand, value }),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    let bids: Vec<Bid> = input
        .trim()
        .lines()
        .map(|l| Bid::from_string(l))
        .flatten()
        .collect();

    println!("Part 2: {}", part_two(&bids));
}

fn part_two(bids: &Vec<Bid>) -> i64 {
    let mut bids: Vec<Bid> = bids.to_vec();
    bids.sort_by(|a, b| a.hand.cmp(&b.hand));
    println!("{:?}", bids[0]);
    println!("{:?}", bids.iter().last().unwrap());
    bids.iter()
        .enumerate()
        .map(|(i, b)| b.value * (i as i64 + 1))
        .sum()
}


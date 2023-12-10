use regex::Regex;
use std::collections::HashSet;
use std::fs;

struct Card {
    id: i32,
    winning_numbers: HashSet<i32>,
    numbers: Vec<i32>,
}

impl Card {
    fn from_string(game_str: &str) -> Self {
        let re = Regex::new(r"^Card\s+(?<card_id>\d+):(?<w_nums>[^|]+)\|(?<nums>.+)$").unwrap();
        let captures = re.captures(game_str).unwrap();
        let id: i32 = captures.name("card_id").unwrap().as_str().parse().unwrap();
        let w_nums = captures.name("w_nums").unwrap().as_str().trim();
        let w_nums: HashSet<i32> = w_nums
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        let nums = captures.name("nums").unwrap().as_str().trim();
        let nums: Vec<i32> = nums
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();
        Card {
            id,
            winning_numbers: w_nums,
            numbers: nums,
        }
    }

    fn matches_count(self: &Self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count()
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    let cards: Vec<Card> = input
        .lines()
        .map(|line| Card::from_string(line.trim()))
        .collect();

    println!("Part 1: {}", part_one(&cards));
    println!("Part 2: {}", part_two(&cards));
}

fn part_one(cards: &Vec<Card>) -> i32 {
    let mut sum = 0;
    for card in cards {
        let m_cnt = card.matches_count();
        if m_cnt > 0 {
            sum += i32::pow(2, m_cnt as u32 - 1);
        }
    }
    sum
}

fn part_two(cards: &Vec<Card>) -> i32 {
    let mut score: Vec<i32> = Vec::new();
    for card in cards.iter().rev() {
        let m_cnt = card.matches_count();
        score.push(1 + score.iter().rev().take(m_cnt).sum::<i32>())
    }
    score.iter().sum()
}

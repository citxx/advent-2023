use regex::Regex;
use std::cmp::max;
use std::fs;

fn part_one(input: &str) -> i32 {
    let game_re = Regex::new(r"^Game\s+(?<game_id>\d+):(?<rounds>([^;]+)(;[^;]+)+)$").unwrap();

    let mut sum = 0;
    'line: for line in input.lines().map(|l| l.trim()) {
        let captures = game_re.captures(line).unwrap();
        let game_id: i32 = captures.name("game_id").unwrap().as_str().parse().unwrap();
        let rounds = captures.name("rounds").unwrap().as_str().split(";");
        let draws = rounds.flat_map(|r| r.split(",")).map(|draw| draw.trim());
        for draw in draws {
            let mut parts = draw.split(" ");
            let count: i32 = parts.next().unwrap().parse().unwrap();
            let color = parts.next().unwrap();
            if color == "red" && count > 12
                || color == "green" && count > 13
                || color == "blue" && count > 14
            {
                continue 'line;
            }
        }
        sum += game_id;
    }
    return sum;
}

fn part_two(input: &str) -> i32 {
    let game_re = Regex::new(r"^Game\s+(?<game_id>\d+):(?<rounds>([^;]+)(;[^;]+)+)$").unwrap();

    let mut sum = 0;
    for line in input.lines().map(|l| l.trim()) {
        let captures = game_re.captures(line).unwrap();
        let rounds = captures.name("rounds").unwrap().as_str().split(";");
        let draws = rounds.flat_map(|r| r.split(",")).map(|draw| draw.trim());
        let (mut max_r, mut max_g, mut max_b) = (0, 0, 0);
        for draw in draws {
            let mut parts = draw.split(" ");
            let count: i32 = parts.next().unwrap().parse().unwrap();
            let color = parts.next().unwrap();
            match color {
                "red" => max_r = max(max_r, count),
                "green" => max_g = max(max_g, count),
                "blue" => max_b = max(max_b, count),
                _ => panic!(),
            }
        }
        sum += max_r * max_g * max_b;
    }
    return sum;
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let input = input.trim();

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

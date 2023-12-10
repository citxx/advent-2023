use std::fs;

fn parse_int_list(s: &str) -> Vec<i64> {
    s.trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    if let &[t_str, d_str] = &lines[..] {
        let time: Vec<i64> = parse_int_list(t_str.strip_prefix("Time:").unwrap());
        let dist: Vec<i64> = parse_int_list(d_str.strip_prefix("Distance:").unwrap());
        let time_dist: Vec<(i64, i64)> =
            std::iter::zip(time.into_iter(), dist.into_iter()).collect();
        println!("Part 1: {}", part_one(&time_dist));

        let t = t_str
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        let d = d_str
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        println!("Part 2: {}", part_two(t, d));
    }
}

fn part_one(time_dist: &Vec<(i64, i64)>) -> i64 {
    let mut combs = 1;
    for (t, best_d) in time_dist.iter() {
        let ways_to_win = (0..=*t)
            .map(|wait| wait * (t - wait))
            .filter(|d| d > best_d)
            .count();
        combs *= ways_to_win as i64;
    }
    combs
}

fn part_two(time: i64, dist: i64) -> i64 {
    let mut ways_to_lose = (0..=time)
        .map(|wait| wait * (time - wait))
        .take_while(|d| d <= &dist)
        .count() as i64;
    if ways_to_lose * 2 < time + 1 {
        ways_to_lose *= 2
    }
    time + 1 - ways_to_lose
}

use std::collections::HashSet;
use std::fs;

const DX: [i32; 8] = [1, 1, 1, 0, -1, -1, -1, 0];
const DY: [i32; 8] = [-1, 0, 1, 1, 1, 0, -1, -1];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    let scheme = input.lines().map(|line| line.trim()).collect();

    println!("Part 1: {}", part_one(&scheme));
    println!("Part 2: {}", part_two(&scheme));
}

fn part_one(scheme: &Vec<&str>) -> i32 {
    let mut part_number: Vec<Vec<bool>> = Vec::with_capacity(scheme.len());
    for line in scheme {
        part_number.push(vec![false; line.len()]);
    }
    for (i, line) in scheme.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                '0'..='9' | '.' => (),
                _ => {
                    for k in 0..8 {
                        mark_part_number(
                            scheme,
                            &mut part_number,
                            (i as i32) + DX[k as usize],
                            (j as i32) + DY[k as usize],
                        );
                    }
                }
            }
        }
    }
    let mut sum = 0;
    for (i, row) in part_number.iter().enumerate() {
        for (j, is_part_number) in row.iter().enumerate() {
            if *is_part_number {
                let number: String = scheme[i][j..]
                    .chars()
                    .take_while(|c| c.is_digit(10))
                    .collect();
                let number: i32 = number.parse().unwrap();
                sum += number;
            }
        }
    }
    sum
}

fn mark_part_number(scheme: &Vec<&str>, part_number: &mut Vec<Vec<bool>>, i: i32, j: i32) {
    let n = scheme.len() as i32;
    let m = scheme[0].len() as i32;
    if i < 0 || n <= i || j < 0 || m <= j {
        return;
    }

    let i = i as usize;
    let j = j as usize;

    if scheme[i].chars().nth(j).unwrap().is_digit(10) {
        let mut k = j;
        while k > 0 && scheme[i].chars().nth(k - 1).unwrap().is_digit(10) {
            k -= 1;
        }
        part_number[i][k] = true;
    }
}

fn part_two(scheme: &Vec<&str>) -> i32 {
    let mut part_number: Vec<Vec<bool>> = Vec::with_capacity(scheme.len());
    for line in scheme {
        part_number.push(vec![false; line.len()]);
    }
    let mut sum = 0;
    for (i, line) in scheme.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            let gear_ratio = match c {
                '*' => find_gear_ratio(scheme, i, j),
                _ => None,
            };
            match gear_ratio {
                Some(r) => sum += r,
                None => (),
            }
        }
    }
    sum
}

fn find_gear_ratio(scheme: &Vec<&str>, i: usize, j: usize) -> Option<i32> {
    let mut connected_parts: Vec<i32> = Vec::new();
    let n = scheme.len() as i32;
    let m = scheme[0].len() as i32;
    let mut processed_xy: HashSet<(usize, usize)> = HashSet::new();
    for (dx, dy) in std::iter::zip(DX, DY) {
        let x = (i as i32) + dx;
        let y = (j as i32) + dy;
        if x < 0 || n <= x || y < 0 || m <= y {
            continue;
        }
        let x = x as usize;
        let y = y as usize;
        if processed_xy.contains(&(x, y)) {
            continue;
        }
        match find_part_number_and_range(scheme, x, y) {
            Some((part_num, (y_begin, y_end))) => {
                connected_parts.push(part_num);
                for ny in y_begin..y_end {
                    processed_xy.insert((x, ny));
                }
            },
            None => (),
        }
    }
    match connected_parts.len() {
        2 => Some(connected_parts[0] * connected_parts[1]),
        _ => None,
    }
}

fn find_part_number_and_range(scheme: &Vec<&str>, i: usize, j: usize) -> Option<(i32, (usize, usize))> {
    if !scheme[i].chars().nth(j).unwrap().is_digit(10) {
        return None;
    }
    let mut k = j;
    while k > 0 && scheme[i].chars().nth(k - 1).unwrap().is_digit(10) {
        k -= 1;
    }
    let num_str: String = scheme[i].chars().skip(k).take_while(|c| c.is_digit(10)).collect();
    let num_len = num_str.chars().count();
    Some((num_str.parse::<i32>().unwrap(), (k, k + num_len)))
}

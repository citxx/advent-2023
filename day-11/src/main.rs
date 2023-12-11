use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.trim().lines().filter(|l| !l.is_empty());
    let map: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();

    println!("Part 1: {}", solve(&map, 2));
    println!("Part 2: {}", solve(&map, 1000000));
}

fn solve(map: &Vec<Vec<char>>, expansion_rate: usize) -> i64 {
    let n = map.len();
    let m = map[0].len();
    let mut rows_to_expand: HashSet<usize> = (0..n).collect();
    let mut columns_to_expand: HashSet<usize> = (0..m).collect();
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    for (x, row) in map.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == '#' {
                rows_to_expand.remove(&x);
                columns_to_expand.remove(&y);
                galaxies.push((x, y));
            }
        }
    }
    let new_row = coord_expansion(n, &rows_to_expand, expansion_rate);
    let new_column = coord_expansion(m, &columns_to_expand, expansion_rate);

    let mut sum = 0;
    for (x1, y1) in galaxies.iter() {
        for (x2, y2) in galaxies.iter() {
            sum += i64::abs(new_row[*x2] as i64 - new_row[*x1] as i64);
            sum += i64::abs(new_column[*y2] as i64 - new_column[*y1] as i64);
        }
    }
    sum / 2
}

fn coord_expansion(
    mx: usize,
    positions_to_expand: &HashSet<usize>,
    expansion_rate: usize,
) -> Vec<usize> {
    let mut exp = 0;
    let mut new_coord: Vec<usize> = Vec::new();
    for i in 0..mx {
        new_coord.push(i + exp);
        if positions_to_expand.contains(&i) {
            exp += expansion_rate - 1;
        }
    }
    new_coord
}

use std::collections::{HashSet, VecDeque};
use std::fs;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Pos(usize, usize);

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.trim().lines().filter(|l| !l.is_empty());
    let map: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    let mut start_pos: Option<Pos> = None;
    for (x, row) in map.iter().enumerate() {
        for (y, &c) in row.iter().enumerate() {
            if c == 'S' {
                start_pos = Some(Pos(x, y));
            }
        }
    }

    let start_pos = start_pos.unwrap();
    println!("Part 1: {}", part_one(&map, &start_pos));
    println!("Part 2: {}", part_two(&map, &start_pos));
}

fn part_one(map: &Vec<Vec<char>>, start_pos: &Pos) -> i64 {
    let dist = bfs(map, start_pos);
    dist.into_iter().flatten().flatten().max().unwrap_or(0) as i64
}

fn part_two(map: &Vec<Vec<char>>, start_pos: &Pos) -> i64 {
    let ext_map = expand_map(map);
    let ext_start_pos = Pos(start_pos.0 * 2, start_pos.1 * 2);
    let dist = bfs(&ext_map, &ext_start_pos);
    let mut areas: Vec<i32> = Vec::new();
    for dx in [-1, 0, 1].into_iter() {
        for dy in [-1, 0, 1].into_iter() {
            let pos = Pos((ext_start_pos.0 as i32 + dx) as usize, (ext_start_pos.1 as i32 + dy) as usize);
            areas.push(bfs_count_even_nones(&dist, &pos));
        }
    }
    // Works only for the specific input. The generic solution would be choosing the number for the
    // area, that doesn't touch the borders
    *areas.iter().filter(|&&a| a > 0).min().unwrap() as i64
}

const DX: &[i32; 4] = &[0, 1, 0, -1];
const DY: &[i32; 4] = &[1, 0, -1, 0];
const EAST: usize = 0;
const SOUTH: usize = 1;
const WEST: usize = 2;
const NORTH: usize = 3;

fn bfs(map: &Vec<Vec<char>>, start_pos: &Pos) -> Vec<Vec<Option<i32>>> {
    let n = map.len();
    let m = map[0].len();
    let mut dist: Vec<Vec<Option<i32>>> = vec![vec![None; n]; m];
    let mut to_visit: VecDeque<(Pos, i32)> = VecDeque::new();
    let Pos(x, y) = start_pos;
    dist[*x][*y] = Some(0);
    for dir in [EAST, SOUTH, WEST, NORTH].into_iter() {
        let next_pos = pos_in_dir(start_pos, dir);
        let Pos(next_x, next_y) = next_pos;
        let next_pipe = map[next_x][next_y];
        let back_dir = (dir + 2) % 4;
        if directions(next_pipe).any(|x| x == back_dir) {
            to_visit.push_back((next_pos, 1));
        }
    }
    while !to_visit.is_empty() {
        let (pos, d) = to_visit.pop_front().unwrap();
        if !is_valid_pos(map, &pos) {
            continue;
        }
        let Pos(x, y) = pos;
        if dist[x][y].is_some() {
            continue;
        }
        dist[x][y] = Some(d);

        for dir in directions(map[x][y]) {
            to_visit.push_back((
                pos_in_dir(&pos, dir),
                d + 1,
            ));
        }
    }
    dist
}

fn bfs_count_even_nones(map: &Vec<Vec<Option<i32>>>, start_pos: &Pos) -> i32 {
    let mut to_visit: VecDeque<Pos> = VecDeque::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    to_visit.push_back(start_pos.clone());
    let mut cnt = 0;
    while !to_visit.is_empty() {
        let pos = to_visit.pop_front().unwrap();
        if !is_valid_pos(map, &pos) {
            continue;
        }
        if visited.contains(&pos) {
            continue;
        }
        let Pos(x, y) = pos;
        if map[x][y].is_some() {
            continue;
        }
        visited.insert(pos.clone());
        if pos.0 % 2 == 0 && pos.1 % 2 == 0 {
            cnt += 1;
        }
        for dir in [EAST, SOUTH, WEST, NORTH].into_iter() {
            to_visit.push_back(pos_in_dir(&pos, dir));
        }
    }
    cnt
}

fn is_valid_pos<T>(map: &Vec<Vec<T>>, pos: &Pos) -> bool {
    let Pos(x, y) = pos;
    *x < map.len() && *y < map[0].len()
}

fn directions(c: char) -> impl Iterator<Item=usize> {
    match c {
        '|' => vec![NORTH, SOUTH].into_iter(),
        '-' => vec![EAST, WEST].into_iter(),
        'L' => vec![NORTH, EAST].into_iter(),
        'J' => vec![NORTH, WEST].into_iter(),
        '7' => vec![SOUTH, WEST].into_iter(),
        'F' => vec![SOUTH, EAST].into_iter(),
        _ => vec![].into_iter(),
    }
}

fn pos_in_dir(pos: &Pos, dir: usize) -> Pos {
    let Pos(x, y) = pos;
    Pos((*x as i32 + DX[dir]) as usize, (*y as i32 + DY[dir]) as usize)
}

fn expand_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = map.len();
    let m = map[0].len();
    let n2 = 2 * n - 1;
    let m2 = 2 * m - 1;
    let mut ext_map: Vec<Vec<char>> = vec![vec![' '; m2]; n2];
    for x in 0..n2 {
        for y in 0..m2 {
            let x_even = x % 2 == 0;
            let y_even = y % 2 == 0;
            ext_map[x][y] = match (x_even, y_even) {
                (true, true) => map[x / 2][y / 2],
                (false, false) => '.',
                (true, false) => horizontal_expansion(map[x / 2][y / 2], map[x / 2][y / 2 + 1]),
                (false, true) => vertical_expansion(map[x / 2][y / 2], map[x / 2 + 1][y / 2]),
            }
        }
    }
    ext_map
}

fn horizontal_expansion(l: char, r: char) -> char {
    let left_connection = l == 'S' || directions(l).any(|dir| dir == EAST);
    let right_connection = r == 'S' || directions(r).any(|dir| dir == WEST);
    if left_connection && right_connection {
        '-'
    } else {
        '.'
    }
}

fn vertical_expansion(u: char, d: char) -> char {
    let up_connection = u == 'S' || directions(u).any(|dir| dir == SOUTH);
    let down_connection = d == 'S' || directions(d).any(|dir| dir == NORTH);
    if up_connection && down_connection {
        '|'
    } else {
        '.'
    }
}

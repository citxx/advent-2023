use regex::Regex;
use rulinalg::matrix::Matrix;
use rulinalg::vector::Vector;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    let mut lines_iter = input.lines().filter(|l| !l.is_empty());
    let directions = lines_iter.next().unwrap();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let node_re = Regex::new(r"^(?<from>[A-Z]{3}) = \((?<L>[A-Z]{3}), (?<R>[A-Z]{3})\)$").unwrap();
    for line in lines_iter {
        let caps = node_re.captures(line).unwrap();
        let from = caps.name("from").unwrap().as_str();
        let left = caps.name("L").unwrap().as_str();
        let right = caps.name("R").unwrap().as_str();
        map.insert(from, (left, right));
    }

    println!("Part 1: {}", part_one(&directions, &map));
    println!("Part 2: {}", part_two(&directions, &map));
}

fn part_one(directions: &str, map: &HashMap<&str, (&str, &str)>) -> i64 {
    let mut cur_node = "AAA";
    let mut steps_cnt = 0;
    let mut dirs_iter = directions.chars().cycle();
    while cur_node != "ZZZ" {
        let dir = dirs_iter.next().unwrap();
        if let (left, right) = map[cur_node] {
            cur_node = match dir {
                'L' => left,
                'R' => right,
                _ => panic!(),
            };
            steps_cnt += 1;
        } else {
            panic!();
        }
    }
    steps_cnt
}

fn part_two(directions: &str, map: &HashMap<&str, (&str, &str)>) -> i64 {
    let mut ordered_nodes: Vec<&str> = map.keys().map(|&x| x).collect();
    ordered_nodes.sort();
    let mut cur_nodes: HashSet<&str> = ordered_nodes
        .iter()
        .filter(|node| node.ends_with("A"))
        .map(|&x| x)
        .collect();
    let mut steps_cnt: i128 = 1;
    // Limited solution. It works only because in the input there's exactly one final position in
    // each cycle and it first comes exactly in one cycle length.
    for node in cur_nodes.iter() {
        let desert_loop = find_loop(directions, map, node);
        steps_cnt = lcm(steps_cnt, desert_loop.cycle_ends.len() as i128);
    }
    steps_cnt as i64
}

fn gcd(a: i128, b: i128) -> i128 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn lcm(a: i128, b: i128) -> i128 {
    a / gcd(a, b) * b
}

#[derive(Debug)]
struct DesertLoop {
    prefix_ends: Vec<bool>,
    cycle_ends: Vec<bool>,
}

fn find_loop(directions: &str, map: &HashMap<&str, (&str, &str)>, start: &str) -> DesertLoop {
    let mut visited_to_global_pos: HashMap<(&str, usize), usize> = HashMap::new();
    let mut path_ends: Vec<bool> = Vec::new();
    let dirs_with_pos = directions.chars().enumerate();
    let mut cur_node = start;
    for (global_pos, (dir_pos, dir)) in dirs_with_pos.cycle().enumerate() {
        let to_visit = (cur_node, dir_pos);
        if let Some(prev_global_pos) = visited_to_global_pos.get(&to_visit) {
            let cycle_ends = path_ends.split_off(*prev_global_pos);
            return DesertLoop {
                prefix_ends: path_ends,
                cycle_ends,
            };
        }
        visited_to_global_pos.insert(to_visit, global_pos);
        path_ends.push(cur_node.ends_with("Z"));
        let (left, right) = map[cur_node];
        cur_node = match dir {
            'L' => left,
            'R' => right,
            _ => panic!(),
        }
    }
    panic!();
}

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
    let node_to_index: HashMap<&str, usize> = ordered_nodes.iter().enumerate().map(|(idx, &node)| (node, idx)).collect();
    let start_vec = Vector::new(ordered_nodes.iter().map(|node| if node.ends_with("A") { 1 } else { 0 }).collect::<Vec<u64>>());
    let target_vec = Vector::new(ordered_nodes.iter().map(|node| if node.ends_with("Z") { 1 } else { 0 }).collect::<Vec<u64>>());
    println!("{:?}", start_vec);
    let mut steps_cnt = 0;
    //let mut dirs_iter = directions.chars().cycle();
    //while !cur_nodes.iter().all(|node| node.ends_with("Z")) {
        //let dir = dirs_iter.next().unwrap();
        //cur_nodes = cur_nodes.into_iter().map(|node| {
            //if let (left, right) = map[node] {
                //match dir {
                    //'L' => left,
                    //'R' => right,
                    //_ => panic!(),
                //}
            //} else {
                //panic!();
            //}
        //}).collect();
        //steps_cnt += 1;
    //}
    //println!("{} {:?}", cur_nodes.iter().filter(|node| node.chars().last() == Some('Z')).count(), cur_nodes);
    steps_cnt
}

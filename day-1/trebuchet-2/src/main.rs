#[macro_use]
extern crate maplit;

use std::collections::HashMap;
use std::fs;

fn main() {
    let w2d: HashMap<&str, i32> = hashmap! {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
    };

    let input = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines().filter(|s| !s.is_empty()) {
        let first_match_index = w2d
            .keys()
            .map(|w| line.find(w))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .min()
            .unwrap();
        let last_match_index = w2d
            .keys()
            .map(|w| line.rfind(w))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .max()
            .unwrap();
        let first_sub = &line[first_match_index..];
        let last_sub = &line[last_match_index..];
        let (_, first_match) = w2d
            .iter()
            .filter(|(k, _v)| first_sub.starts_with(*k))
            .next()
            .unwrap();
        let (_, last_match) = w2d
            .iter()
            .filter(|(k, _v)| last_sub.starts_with(*k))
            .next()
            .unwrap();
        let calibration_value = first_match * 10 + last_match;
        sum += calibration_value;
        //println!("{line}: {first_match} | {last_match} | {calibration_value}");
    }
    println!("{sum}");
}

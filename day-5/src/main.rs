use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::fs;

#[derive(Debug, Clone)]
struct Interval {
    start: i64,
    end: i64,
}

#[derive(Debug)]
struct IntervalMap {
    from: Interval,
    to_start: i64,
}

#[derive(Debug)]
struct IntervalMapping {
    from: String,
    to: String,
    sorted_interval_maps: Vec<IntervalMap>,
}

impl Interval {
    fn from_boundaries(start: i64, end: i64) -> Option<Self> {
        if start < end {
            Some(Interval { start, end })
        } else {
            None
        }
    }

    fn split_with(self: &Self, other: &Self) -> (Option<Self>, Option<Self>, Option<Self>) {
        let before = Interval::from_boundaries(self.start, min(self.end, other.start));
        let middle =
            Interval::from_boundaries(max(self.start, other.start), min(self.end, other.end));
        let after = Interval::from_boundaries(max(self.start, other.end), self.end);
        (before, middle, after)
    }
}

impl IntervalMap {
    fn from_string(s: &str) -> Self {
        let nums: Vec<i64> = s
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        match &nums[..] {
            &[to_start, from_start, len] => Self {
                from: Interval {
                    start: from_start,
                    end: from_start + len,
                },
                to_start,
            },
            _ => panic!(),
        }
    }

    fn convert(self: &Self, value: i64) -> Option<i64> {
        if self.from.start <= value && value < self.from.end {
            Some(value - self.from.start + self.to_start)
        } else {
            None
        }
    }

    fn convert_interval(
        self: &Self,
        int: &Interval,
    ) -> (Option<Interval>, Option<Interval>, Option<Interval>) {
        let (before, middle, after) = int.split_with(&self.from);
        let converted_middle = middle.map(|m| {
            let conv_m_start = self.convert(m.start).unwrap();
            Interval {
                start: conv_m_start,
                end: conv_m_start + (m.end - m.start),
            }
        });
        (before, converted_middle, after)
    }
}

impl IntervalMapping {
    fn convert_values_into(self: &Self, values: Vec<i64>) -> Vec<i64> {
        let mut values = values.clone();
        values.sort();
        let mut result: Vec<i64> = Vec::new();
        let mut int_iter = self.sorted_interval_maps.iter();
        let mut cur_int = int_iter.next();
        for v in values.into_iter() {
            while cur_int.is_some() && cur_int.unwrap().from.end <= v {
                cur_int = int_iter.next();
            }
            let converted = cur_int.map_or(v, |int| int.convert(v).unwrap_or(v));
            result.push(converted);
        }
        result
    }

    fn convert_intervals_into(self: &Self, mut intervals: Vec<Interval>) -> Vec<Interval> {
        let mut result: Vec<Interval> = Vec::new();
        intervals.sort_by(|a, b| a.start.cmp(&b.start));
        let mut int_iter = intervals.iter();
        let mut map_iter = self.sorted_interval_maps.iter();
        let mut cur_int_owned: Option<Interval>;
        let mut cur_int = int_iter.next();
        let mut cur_map = map_iter.next();
        while let Some(int_to_convert) = cur_int {
            if let Some(map) = cur_map {
                let (before, middle, after) = map.convert_interval(int_to_convert);
                if let Some(b) = before {
                    result.push(b);
                }
                if let Some(m) = middle {
                    result.push(m);
                }
                if let Some(_) = after {
                    cur_int_owned = after;
                    cur_int = cur_int_owned.as_ref();
                    cur_map = map_iter.next();
                    continue;
                } else {
                    cur_int = int_iter.next();
                    continue;
                }
            }
            result.push(int_to_convert.clone());
            cur_int = int_iter.next();
        }
        result
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    let mut lines_iter = input.lines().filter(|l| !l.is_empty()).peekable();
    let seeds = lines_iter.next().unwrap().strip_prefix("seeds: ").unwrap();
    let seeds: Vec<i64> = seeds
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut mappings: Vec<IntervalMapping> = Vec::new();
    let mapping_re = Regex::new(r"^(?<from>[a-z]+)-to-(?<to>[a-z]+) map:$").unwrap();
    loop {
        let (from, to) = match lines_iter.next() {
            None => break,
            Some(title) => {
                let caps = mapping_re.captures(title).unwrap();
                (
                    caps.name("from").unwrap().as_str().to_string(),
                    caps.name("to").unwrap().as_str().to_string(),
                )
            }
        };
        let int_str_iter =
            std::iter::from_fn(|| lines_iter.next_if(|l| l.chars().next().unwrap().is_digit(10)));
        let mut intervals: Vec<IntervalMap> =
            int_str_iter.map(|l| IntervalMap::from_string(l)).collect();
        intervals.sort_by(|a, b| a.from.start.cmp(&b.from.start));
        mappings.push(IntervalMapping {
            from,
            to,
            sorted_interval_maps: intervals,
        })
    }

    println!("Part 1: {}", part_one(&seeds, &mappings));
    println!("Part 2: {}", part_two(&seeds, &mappings));
}

fn part_one(seeds: &Vec<i64>, mappings: &Vec<IntervalMapping>) -> i64 {
    let mut values = seeds.clone();
    for mapping in mappings.iter() {
        values = mapping.convert_values_into(values);
    }
    *values.iter().min().unwrap()
}

fn part_two(seeds: &Vec<i64>, mappings: &Vec<IntervalMapping>) -> i64 {
    let mut intervals: Vec<Interval> = Vec::new();
    let mut seeds_iter = seeds.iter();
    loop {
        let start_opt = seeds_iter.next();
        let len_opt = seeds_iter.next();
        if let (Some(start), Some(len)) = (start_opt, len_opt) {
            intervals.push(Interval {
                start: *start,
                end: start + len,
            })
        } else {
            break;
        }
    }
    for mapping in mappings.iter() {
        println!("{:?}", intervals);
        intervals = mapping.convert_intervals_into(intervals);
        println!("{:?}", mapping.sorted_interval_maps);
    }
    println!("{:?}", intervals);
    intervals.into_iter().map(|int| int.start).min().unwrap()
}

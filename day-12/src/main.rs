use std::collections::HashMap;
use std::fs;

type Groups = Vec<i64>;

#[derive(Debug)]
struct Record {
    state: Vec<char>,
    broken_groups: Groups,
}

impl Record {
    fn from_str(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if let &[state_str, groups_str] = &parts[..] {
            let state: Vec<char> = state_str.chars().collect();
            let groups_opt: Vec<Option<usize>> = groups_str.split(",").map(|s| s.parse::<usize>().ok()).collect();
            if groups_opt.iter().any(|x| x.is_none()) {
                None
            } else {
                Some(Record {
                    state,
                    broken_groups: groups_opt.into_iter().flatten().map(|x| x as i64).collect(),
                })
            }

        } else {
            None
        }
    }

    fn unfolded(&self) -> Self {
        let n_state = self.state.len();
        let n_groups = self.broken_groups.len();
        Record {
            state: self.state.iter().chain(vec!['?'].iter()).cycle().take(n_state * 5 + 4).copied().collect(),
            broken_groups: self.broken_groups.iter().cycle().take(n_groups * 5).copied().collect(),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.trim().lines().filter(|l| !l.is_empty());
    let records: Vec<Record> = lines.map(|l| Record::from_str(l).unwrap()).collect();

    println!("Part 1: {}", part_one(&records));
    println!("Part 2: {}", part_two(&records));
}

fn part_one(records: &Vec<Record>) -> i64 {
    solve(records)
}

fn part_two(records: &Vec<Record>) -> i64 {
    let unfolded_records: Vec<Record> = records.iter().map(|rec| rec.unfolded()).collect();
    solve(&unfolded_records)
}

fn solve(records: &Vec<Record>) -> i64 {
    records.iter().map(|rec| combinations_count(rec)).sum()
}

fn combinations_count(record: &Record) -> i64 {
    let n = record.state.len();
    // Putting -1 in front to indicate that the first group didn't start yet and functioning springs
    // can go before. Reversing for easier handling below.
    let start_groups = record.broken_groups.iter().rev().copied().chain([-1].into_iter()).collect();

    // combs[groups] after iteration i is the number of ways to restore the record on the prefix
    // 0..=i with specific remaining groups.
    let mut combs: HashMap<Groups, i64> = HashMap::from([(start_groups, 1)]);
    for i in 0..n {
        let mut next_combs: HashMap<Groups, i64> = HashMap::new();
        for (groups, cnt) in combs.iter() {
            let mut diff: Vec<Option<(Groups, i64)>> = Vec::new();
            if record.state[i] == '.' || record.state[i] == '?' {
                diff.push(added_combs(false, &groups, *cnt));
            }
            if record.state[i] == '#' || record.state[i] == '?' {
                diff.push(added_combs(true, &groups, *cnt));
            }
            for (next_groups, next_cnt) in diff.into_iter().flatten() {
                let prev_val = next_combs.get(&next_groups).unwrap_or(&0);
                next_combs.insert(next_groups, prev_val + next_cnt);
            }
        }
        combs = next_combs;
    }
    (combs.get(&vec![-1]).unwrap_or(&0) + combs.get(&vec![0]).unwrap_or(&0)) as i64
}

fn added_combs(is_broken: bool, groups: &Groups, cnt: i64) -> Option<(Groups, i64)> {
    let next_groups = if is_broken {
        remove_broken_spring_from_groups(groups)
    } else {
        remove_operational_spring_from_groups(groups)
    };
    match next_groups {
        None => None,
        Some(next_groups) => Some((next_groups, cnt)),
    }
}

fn remove_broken_spring_from_groups(groups: &Groups) -> Option<Groups> {
    if groups.is_empty() {
        return None;
    }
    let mut result: Groups = groups.clone();
    let last = result.pop().unwrap();
    match last {
        0 => None,
        -1 => remove_broken_spring_from_groups(&result),
        _ => {
            result.push(last - 1);
            Some(result)
        }
    }
}

fn remove_operational_spring_from_groups(groups: &Groups) -> Option<Groups> {
    if groups.is_empty() {
        return Some(Vec::new());
    }
    let mut result: Groups = groups.clone();
    let last = result.pop().unwrap();
    match last {
        -1 | 0 => {
            result.push(-1);
            Some(result)
        }
        _ => None,
    }
}

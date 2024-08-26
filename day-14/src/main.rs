use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Platform {
    state: Vec<Vec<char>>,
}

enum Dir {
    NORTH,
    WEST,
    SOUTH,
    EAST,
}

const CYCLE_REPETITIONS: usize = 1000000000;
const FOUND_LOOP_LENGTH: usize = 27;

impl Platform {
    fn shift(&mut self, direction: Dir) -> &mut Self {
        match direction {
            Dir::NORTH => self.shift_vertical(false),
            Dir::WEST => self.shift_horizontal(false),
            Dir::SOUTH => self.shift_vertical(true),
            Dir::EAST => self.shift_horizontal(true),
        }
        self
    }

    fn shift_vertical(&mut self, inverted: bool) {
        for col in 0..self.state[0].len() {
            let mut target = 0;
            for row in 0..self.state.len() {
                let r = if inverted { self.state.len() - 1 - row } else { row };
                let t = if inverted { self.state.len() - 1 - target } else { target };
                match self.state[r][col] {
                    '#' => target = row + 1,
                    'O' => {
                        self.state[r][col] = '.';
                        self.state[t][col] = 'O';
                        target += 1
                    }
                    _ => (),
                }
            }
        }
    }

    fn shift_horizontal(&mut self, inverted: bool) {
        for row in 0..self.state.len() {
            let mut target = 0;
            for col in 0..self.state[row].len() {
                let c = if inverted { self.state[row].len() - 1 - col } else { col };
                let t = if inverted { self.state[row].len() - 1 - target } else { target };
                match self.state[row][c] {
                    '#' => target = col + 1,
                    'O' => {
                        self.state[row][c] = '.';
                        self.state[row][t] = 'O';
                        target += 1
                    }
                    _ => (),
                }
            }
        }
    }

    fn load_north(&self) -> i64 {
        let mut load: i64 = 0;
        let size = self.state.len();
        for (i, row) in self.state.iter().enumerate() {
            for c in row.iter() {
                load += match c {
                    'O' => (size - i) as i64,
                    _ => 0,
                }
            }
        }
        load
    }

    fn fingerprint(&self) -> i64 {
        let mut val: i64 = 1;
        for c in self.state.iter().flatten() {
            val = (val * 400003171 + (*c == 'O') as i64) % 1000001213;
        }
        val
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.trim().lines().filter(|l| !l.is_empty());
    let platform = Platform {
        state: lines.map(|l| l.chars().collect()).collect(),
    };

    println!("Part 1: {}", part_one(&platform));
    find_loop(&platform);
    println!("Part 2: {}", part_two(&platform));
}

fn part_one(platform: &Platform) -> i64 {
    let mut p = platform.clone();
    p.shift_vertical(false);
    p.load_north()
}

fn part_two(platform: &Platform) -> i64 {
    let mut p = platform.clone();
    let repetitions = CYCLE_REPETITIONS % FOUND_LOOP_LENGTH + 10 * FOUND_LOOP_LENGTH;
    for i in 0..repetitions {
        p.shift(Dir::NORTH).shift(Dir::WEST).shift(Dir::SOUTH).shift(Dir::EAST);
    }
    p.load_north()
}

fn find_loop(platform: &Platform) {
    let mut p = platform.clone();
    let mut m: HashMap<i64, usize> = HashMap::new();
    let mut i: usize = 0;
    let mut cycle_len: usize = 0;
    let mut cycle_repetitions: usize = 0;
    while cycle_len == 0 || cycle_repetitions < cycle_len {
        // Check cycle
        let f = p.fingerprint();
        if m.contains_key(&f) {
            let len = i - m[&f];
            if len == cycle_len {
                cycle_repetitions += 1;
            } else {
                cycle_len = len;
                cycle_repetitions = 1;
            }
            println!("Match on interation {}, cycle length {}, sustained for {}", i, len, cycle_repetitions);
        } else {
            cycle_len = 0;
            cycle_repetitions = 0;
        }
        m.insert(f, i);

        // Iterate
        p.shift(Dir::NORTH).shift(Dir::WEST).shift(Dir::SOUTH).shift(Dir::EAST);
        i += 1;
    }
    println!("Found cycle of length {}, first ending on iteration {}", cycle_len, i);
}

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut sum = 0;
    for line in input.split('\n').filter(|s| !s.is_empty()) {
        let digits: Vec<u32> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        sum += digits[0] * 10 + digits.last().unwrap();
    }
    println!("{sum}");
}

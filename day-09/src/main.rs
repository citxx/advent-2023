use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.trim().lines();

    let seqs: Vec<Vec<i64>> = lines
        .map(|l| {
            l.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    let rev_seqs: Vec<Vec<i64>> = seqs
        .iter()
        .map(|seq| seq.iter().rev().map(|&x| x).collect())
        .collect();

    println!("Part 1: {}", sum_of_prevs(&rev_seqs));
    println!("Part 2: {}", sum_of_prevs(&seqs));
}

fn sum_of_prevs(seqs: &Vec<Vec<i64>>) -> i64 {
    let mut sum: i64 = 0;
    for seq in seqs.iter() {
        let n = seq.len() + 1;
        let mut val: Vec<Vec<i64>> = vec![vec![0; n]; n];
        for j in 1..n {
            val[0][j] = seq[j - 1];
        }
        for i in 1..n {
            for j in 1..(n - i) {
                val[i][j] = val[i - 1][j + 1] - val[i - 1][j];
            }
        }
        for i in (0..n - 1).rev() {
            val[i][0] = val[i][1] - val[i + 1][0];
        }
        sum += val[0][0];
    }
    sum
}

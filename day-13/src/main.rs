use std::collections::HashSet;
use std::fs;

struct MirrorField {
    field: Vec<Vec<char>>,
}

impl MirrorField {
    fn reflection_points(line: &Vec<char>) -> Vec<usize> {
        let n = line.len();
        let mut result: Vec<usize> = Vec::new();
        for i in 1..n {
            let is_reflection = std::iter::zip(line[..i].iter().rev(), line[i..].iter()).all(|(a, b)| a == b);
            if is_reflection {
                result.push(i);
            }
        }
        result
    }

    fn horizontal_reflections(field: &Vec<Vec<char>>) -> Vec<usize> {
        let m = field[0].len();
        let mut common_points: HashSet<usize> = HashSet::from_iter(1..m);
        for line in field.iter() {
            let ref_points = HashSet::<usize>::from_iter(Self::reflection_points(line).iter().copied());
            common_points = common_points.intersection(&ref_points).copied().collect();
        }
        common_points.into_iter().collect()
    }

    fn reflections_sum(&self) -> usize {
        let h_refs = Self::horizontal_reflections(&self.field);
        let v_refs = Self::horizontal_reflections(&self.transposed_field());
        h_refs.into_iter().sum::<usize>() + 100 * v_refs.into_iter().sum::<usize>()
    }

    fn transposed_field(&self) -> Vec<Vec<char>> {
        let n = self.field.len();
        let m = self.field[0].len();
        let mut result: Vec<Vec<char>> = Vec::new();
        for i in 0..m {
            result.push((0..n).map(|j| self.field[j][i]).collect());
        }
        result
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut fields: Vec<MirrorField> = Vec::new();
    // TODO: look if there's a split/group for iterators.
    let mut field: Vec<Vec<char>> = Vec::new();
    for line in input.trim().lines().chain([""].into_iter()) {
        if line.is_empty() {
            fields.push(MirrorField {field});
            field = Vec::new();
        } else {
            field.push(line.chars().collect());
        }
    }

    println!("Part 1: {}", part_one(&fields));
}

fn part_one(fields: &Vec<MirrorField>) -> i64 {
    let mut sum = 0;
    for field in fields.iter() {
        sum += field.reflections_sum();
    }
    sum as i64
}


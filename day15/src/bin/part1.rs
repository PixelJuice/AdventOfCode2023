use std::result;

use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> i32 {
    let values = input.split(",").map(|value| {
        value.chars().map(|char| (char as u8) as i32).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>();
    let mut results = vec![];
    for va in values {
        let mut result = 0;
        for c in va {
            result = (result + c) * 17 % 256;
        }
        results.push(result);
    }
    results.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_hash() {
        let result = solve("HASH");
        assert_eq!(result, 52);
    }

    #[test]
    fn solve_score() {
        let result = solve(include_str!("./input_test.txt"));
        assert_eq!(result, 1320);
    }
}
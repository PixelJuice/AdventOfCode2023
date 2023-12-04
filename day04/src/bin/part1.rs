use std::collections::HashSet;


fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let mut value:u32 = 0;
    for line in input.lines() {
        let first_split = line.split_once(':').unwrap();
        let second_split = first_split.1.split_once('|').unwrap();
        let winning_numbers:HashSet<u32> = second_split.0.trim().split_ascii_whitespace().map(|num| num.parse::<u32>().expect("only numbers")).collect();
        let ticket_numbers:HashSet<u32> = second_split.1.trim().split_ascii_whitespace().map(|num| num.parse::<u32>().expect("only numbers")).collect();
        let mut win_value:u32 = 0;
        let wins:u32 = ticket_numbers.intersection(&winning_numbers).count() as u32;
        if wins > 0 {
            win_value = (2 as u32).pow(wins - 1);
        }    
        value += win_value;
    }
    
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let result = solve("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, "13");
    }
}
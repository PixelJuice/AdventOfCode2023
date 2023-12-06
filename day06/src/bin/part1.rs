fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let output = input;
    output.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let result = solve("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(result, "288");
    }

    #[test]
    fn solve_actual_puzzle() {
        let result = solve("Time:        40     81     77     72
        Distance:   219   1012   1365   1089");
        assert_eq!(result, result);
    }
}
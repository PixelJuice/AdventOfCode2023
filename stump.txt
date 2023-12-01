fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let result = solve("");
        assert_eq!(result, "");
    }
}
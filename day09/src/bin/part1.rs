
fn main() {
    let input = include_str!("./input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let data = input.lines().map(|line| {
        line.trim().split_ascii_whitespace().map(|str| str.parse::<i32>().expect("numbers but got something else")).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>().iter().map(|vector| {
        vector.iter()..sum::<i32>()
    }).collect::<Vec<i32>>();
    dbg!(data);
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let result = solve("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");
        assert_eq!(result, "114");
    }
}
fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let mut value = 0;
    for line in input.lines() {
        let mut line_numbers: Vec<char> = vec![];
        for letter in line.chars()  {
            if letter.is_numeric() {
                line_numbers.push(letter);
            }
        }
        let first_number = line_numbers[0];
        let second_number = line_numbers[line_numbers.len() -1];
        let mut line_value = first_number.to_string();
        line_value.push(second_number);
        value += line_value.parse::<i32>().unwrap();
    }
    value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let result = solve("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");
        assert_eq!(result, "142");
    }
}
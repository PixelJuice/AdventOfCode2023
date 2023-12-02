fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let mut value = 0; 
    for line in input.lines() {
        let mut line_numbers: Vec<u32> = vec![];
        for letter in line.chars()  {
            match letter.to_digit(10) {
                Some(num) => line_numbers.push(num),
                None => (),
            }
        }
        let first_number = line_numbers.first().expect("should be atleast one number");
        let line_value = match line_numbers.last() {
            Some(x) => format!("{first_number}{x}"),
            None => format!("{first_number}{first_number}"),
        };
        value += line_value.parse::<u32>().expect("this should be a valid u32");
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
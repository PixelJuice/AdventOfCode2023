use std::vec;

fn main() {
    let input = include_str!("./input2.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let mut value = 0;
    for line in input.lines() {
        let numbers = find_number_matches(line.trim());
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();     
        let add_value = format!("{first}{last}").parse::<u32>();
        let as_number = match add_value {
            Ok(x) => x,
            Err(e) => panic!("error {}", e),
        };
        value += as_number;
    }  
    value.to_string()
}

fn find_number_matches(input: &str) -> Vec<u32> {
    let number_in_text: [&str; 9] = ["one","two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut matches: Vec<u32> = vec![];
    for letter in 0..input.len()  {
        let c = input.chars().nth(letter).unwrap();
        if c.is_numeric() {
            matches.push(c.to_digit(10).unwrap());
        }
        else {
            let str_slice = input[letter..].to_owned();
            for number in 0..number_in_text.len() {
                if str_slice.starts_with(number_in_text[number]) {
                    matches.push((number + 1) as u32);
                }
            }
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let result = solve("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");
        assert_eq!(result, "281");
    }

    #[test]
    fn multiple_occurences() {
        let result = solve("45five7fivegpzhcfbbfiveqhnhhzdqtnltgnkrxz");
        assert_eq!(result, "45");
    }
}
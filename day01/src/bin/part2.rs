use std::vec;

fn main() {
    let input = include_str!("./input2.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let mut value = 0;
    for line in input.lines() {
        let mut texts = find_text_matches(line.trim());
        let mut numbers = find_number_matches(line.trim());
        numbers.append(&mut texts);
        numbers.sort_by(|a, b| a.0.cmp(&b.0));
        let mut line_number = numbers[0].1.to_string();
        line_number.push_str(&numbers[numbers.len() - 1].1.to_string().to_owned());
        let add_value = line_number.parse::<u32>();
        let as_number = match add_value {
            Ok(x) => x,
            Err(e) => panic!("error {}", e),
        };
        value += as_number;
        //println!("{}", &as_number);
    }  
    value.to_string()
}

fn find_number_matches(input: &str) -> Vec<(usize, u8)> {
    let mut matches: Vec<(usize, u8)> = vec![];
    for letter in 0..input.len()  {
        let c = input.chars().nth(letter).unwrap();
        if c.is_numeric() {
            matches.push((letter, c.to_digit(10).unwrap() as u8));
        }
    }
    matches
}

fn find_text_matches(input: &str) -> Vec<(usize, u8)> {
    let number_in_text: [&str; 9] = ["one","two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut matches: Vec<(usize, u8)> = vec![];
    for number in 0..number_in_text.len() {
        if input.contains(number_in_text[number]) {
            let collection: Vec<_> = input.match_indices(number_in_text[number]).collect();
            for found in collection  {
                matches.push((found.0, (number + 1) as u8))
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
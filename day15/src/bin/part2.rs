use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focus: u32,
}

fn main() {
    let input = include_str!("./input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    let strings = input.split(",").collect::<Vec<&str>>();
    let mut boxes: HashMap<u32, Vec<Lens<'_>>> = HashMap::new();

    for string in strings.iter() {
        let box_number = hash_box_number(string);
        if string.chars().last().unwrap() == '-'{
            let label = &string[0..string.len()-1];
            match boxes.get_mut(&box_number) {
                Some(v) => {
                    match v.iter().find_position(|v| v.label == label) {
                        Some((index, _)) => {v.remove(index);},
                        None => (),
                    };
                },
                None => (),
            };
            
        } else {
            let focus = match string.chars().last().unwrap().to_digit(10) {
                Some(val) => val,
                None => 0,
            };
            let label = &string[0..string.len()-2];
            if !boxes.contains_key(&box_number) {
                let lens = Lens{label: label, focus: focus};
                boxes.insert(box_number,vec![lens]);
            }
            else {
                let vector = boxes.get_mut(&box_number).expect("should exist");
                match vector.iter().find_position(|v| v.label == label) {
                    Some((k, _v)) => vector[k] = Lens{label,focus},
                    None => vector.push(Lens { label, focus}),
                };
            }

        }
    }
    let mut results = vec![];
    for lens in boxes {
        if lens.1.len() != 0 {
            for (i, v) in lens.1.iter().enumerate() {
                results.push((lens.0 +1) * (i as u32 +1) * v.focus);
            }
        }
    }
    results.iter().sum()
}

fn hash_box_number(input: &str) -> u32 {
    let converted_chars = input.chars().filter(|c| c != &'-' && c != &'=' && !c.is_ascii_digit()).map(|c| c as u8).collect::<Vec<u8>>();
    dbg!(&converted_chars);
    let mut result:u32 = 0;
    for c in converted_chars {
        result = (result + c as u32) * 17 % 256;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_hash() {
        let result = hash_box_number("rn");
        assert_eq!(result, 0);
    }

    #[test]
    fn solve_score() {
        let result = solve(include_str!("./input_test.txt"));
        assert_eq!(result, 145);
    }
}
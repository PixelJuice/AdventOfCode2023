use itertools::{Itertools, repeat_n};



fn main() {
    let input = include_str!("./input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let data = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let new_data = &format!("{}{}", data[0], '?')[..];
        let new_string = repeat_n(new_data, 5).collect::<Vec<&str>>().join("");
        let new_line = &new_string[..new_string.len()-1];
        //dbg!(&new_line);
        let spaces_to_fill = new_line.chars().filter(|c| c == &'?').count() as u32;
        //dbg!(spaces_to_fill);
        let instructions = data[1].split(",").map(|s| s.parse::<u32>().expect("should only be numbers")).collect::<Vec<u32>>();
        let mut new_instructions = vec![]; 
        for _ in 0..5 {
            for t in &instructions {
                new_instructions.push(*t)
            }
        }
        //dbg!(&new_instructions);
        let options = repeat_n([".", "#"].into_iter(), spaces_to_fill as usize).multi_cartesian_product().map(|v| v.join("")).collect::<Vec<String>>();
        for op in options {
            let t = check_option(op, new_line,  &new_instructions);
            if t {
                total += 1;
            }
        }
    }
    total.to_string()
}

fn check_option(op: String, data: &str, instructions: &Vec<u32>) -> bool {
    let mut option_iter = op.chars();
    let test_option = data.chars().map(|c| match c {
        '?' => option_iter.next().expect("it dont knoe"),
        val => val
    }).collect::<String>();
    //dbg!(&test_option);

    let count = test_option.chars().group_by(|c| c == &'#').into_iter().filter_map(|(hash, group)| hash.then_some( group.into_iter().count() as u32)).collect::<Vec<u32>>();
    //dbg!(&count);
    &instructions[..] == &count[..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test_first_row() {
        let result = solve("???.### 1,1,3");
        assert_eq!(result, "1");
    }

    #[test]
    fn solve_test_puzzle() {
        let result = solve(include_str!("./test_input.txt"));
        assert_eq!(result, "525152");
    }
}
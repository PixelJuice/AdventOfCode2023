use std::cmp;

use itertools::{Itertools, repeat_n};
use regex::Regex;


fn main() {
    let input = include_str!("./input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let mut total = 0;
    for line in input.lines() {
        let data = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let spaces_to_fill = line.chars().filter(|c| c == &'?').count() as u32;
        let instructions = data[1].split(",").map(|s| s.parse::<u32>().expect("should only be numbers")).collect::<Vec<u32>>();
        let options = repeat_n([".", "#"].into_iter(), spaces_to_fill as usize).multi_cartesian_product().map(|v| v.join("")).collect::<Vec<String>>();
        //dbg!(&options);
        for op in options {
            let t = check_option(op, data[0],  &instructions);
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


/*fn count(config:String, nums:Vec<u32>) -> u32 {
    dbg!(&config);
    dbg!(&nums);
    if config.is_empty() {
        if nums.is_empty() {
            return 1;
        }
        return 0;
    }
    if nums.is_empty() {
        if config.contains('#') {
            return 0;
        }
        return 1;
    }
    let mut result = 0;

    if config.starts_with(|c: char| c == '.' || c == '?') {
        println!("first char is ? or .");
        result += count(config.chars().collect::<Vec<char>>()[1..].to_vec().iter().join(""), nums.clone());
    }

    if config.starts_with(|c| c == '#' || c == '?') {
        let first: usize = nums[0] as usize;
        /*if first <= config.chars().count() && !config.chars().collect::<Vec<char>>()[0..first].to_vec().contains(&'.') && first == config.chars().count() || config.chars().nth(first).unwrap() != '#' {
            result += count(config.chars().collect::<Vec<char>>()[first..].to_vec().iter().join(""), nums[1..].to_vec())
        }*/

        if first <= config.chars().count() {
            if !config.chars().collect::<Vec<char>>()[0..first].to_vec().contains(&'.') {
                println!("equal {} {}", first, config.chars().count());
                println!("char {}", config.chars().nth(first).is_some());
                let mat = match config.chars().nth(first) {
                    Some(c) => c,
                    None => '#',
                };
                if first == config.chars().count() || mat != '#' {
                    println!("equal");
                    result += count(config.chars().collect::<Vec<char>>().iter().enumerate().filter(|(u, _)|u <= &first).map(|(_, c)| c).join(""), nums[1..].to_vec());
                }
                
            }
            else {
                println!("there is a dot in range");
            }
        }
        else {
            println!("to long to fit anymore");
        }
    }

    result
}*/



/*fn solve2(input: &str) -> String {
    let mut result = 0;
    for line in input.lines() {
        let data = line.split_ascii_whitespace().collect::<Vec<&str>>();
        let instructions = data[1].split(",").map(|s| s.parse::<usize>().expect("should only be numbers")).collect::<Vec<usize>>();
        let reex = regex::Regex::new("\\?+|\\?|\\.+|\\.|#+|#\\B/g").unwrap();
        let reg = reex.find_iter(data[0]).map(|capture| capture.as_str()).collect::<Vec<&str>>();
        let mut permutations: Vec<Vec<String>> = vec![];
        for r in reg {
            if r.starts_with('?') {
                let chars_to_use = ['.', '#',].repeat(cmp::max(r.len(),2)); //hacky but works
                let chars = chars_to_use.iter().permutations(r.len()).unique().collect::<Vec<_>>();
                let joined = chars.into_iter().map(|s | s.iter().join("")).collect::<Vec<String>>();
                //dbg!(&joined);
                permutations.push(joined) 
            }
            else {
                let per = vec![String::from(r)];
                permutations.push(per)
            }
            
        }
        let per = permutations.iter().multi_cartesian_product().map(|f| f.iter().join("")).collect::<Vec<_>>();
        //let mut valid_options = vec![];
        let test = per.iter().filter(|option|check_option(option.to_string(), &instructions)).collect::<Vec<_>>();
        //check_option(per[0].to_string(), &instructions);
        result += test.len();
    }
    //let formatted = format!(r"(\?){{{}}}", result);
    //let re = Regex::new(formatted.as_str()).unwrap();
    //let reg = re.find("??Hello world??").map(|capture| capture);
    //dbg!(reg);
    result.to_string()
}*/

/*fn check_option(option: String, instructions: &Vec<usize>) -> bool {
    let mut work_on = &option[0..option.chars().count()];
    for ins in instructions {
        dbg!(&work_on);
        let formatted = format!(r"(#){{{}}}", ins);
        let re = Regex::new(formatted.as_str()).unwrap();
        let reg = re.find(work_on);
        match reg {
            Some(m) => {
                work_on = &work_on[m.end()..work_on.chars().count()];
                dbg!(m);
            },
            None => return false,
        } 
        if work_on.chars().count() != 0 {
            if !work_on.starts_with('.') {
                println!("contiunue");
                return false
            }
        }  
        
    }
    dbg!(option);
    true
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test_first_row() {
        let result = solve("???.### 1,1,3");
        assert_eq!(result, "0");
    }
/*
    #[test]
    fn test_empty_config() {
        let result = count("".to_string(), vec![1]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_empty_num_but_no_hash() {
        let result = count(".?.".to_string(), vec![]);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_empty_num() {
        let result = count("#?.".to_string(), vec![]);
        assert_eq!(result, 0);
    }

    #[test]
    fn solve_test_first_char_questionmark() {
        let result = solve("????.#...#... 4,1,1");
        assert_eq!(result, "1");
    }*/

    #[test]
    fn solve_test_last_row() {
        let result = solve("?###???????? 3,2,1");
        assert_eq!(result, "10");
    }

    #[test]
    fn solve_test_puzzle() {
        let result = solve(include_str!("./test_input.txt"));
        assert_eq!(result, "21");
    }
}
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let mut current_step = 0;
    let mut current_pos = "AAA";
    let instructions_split = input.split_once("\n").expect("a two split");
    let instructions:Vec<_> = instructions_split.0.trim().chars().map(|instruction| match instruction  {
        'R' => 1,
        'L' => 0,
        val => panic!("should only be left or right {val} instead"),
    }).collect();
    let map = build_map(instructions_split.1.trim());
    
    while current_pos != "ZZZ" {
        let instruction = instructions[current_step % instructions.len()];
        
        current_pos = map.get_key_value(current_pos).expect("a direction").1[instruction];
        current_step += 1;
        println!("curent pos {current_pos}");
    }
    current_step.to_string()
}

fn build_map(instructions_split: &str) -> HashMap<&str, [&str; 2]> {
    let mut map = HashMap::new();
    for line in instructions_split.lines() {
        let key = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        map.insert(key, [left, right]);
    }
    dbg!(&map);
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let result = solve("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)");
        assert_eq!(result, "6");
    }
}
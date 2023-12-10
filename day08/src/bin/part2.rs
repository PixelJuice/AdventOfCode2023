use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let mut current_step = 0;

    let instructions_split = input.split_once("\n").expect("a two split");
    let instructions:Vec<usize> = instructions_split.0.trim().chars().map(|instruction| match instruction  {
        'R' => 1,
        'L' => 0,
        val => panic!("should only be left or right {val} instead"),
    }).collect();

    let data = build_map(instructions_split.1.trim());
    let map = data.0;
    let mut current_pos = data.1;
    let mut test_all = false;
    let mut instruct = instructions.iter().cycle();
    while !test_all {
        let next = instruct.next().unwrap();
        //dbg!(next);
        /*for pos in 0..current_pos.len() {
            current_pos[pos] = map.get_key_value(current_pos[pos]).expect("a direction").1[*next];
        }*/
        current_pos = current_pos.iter().map(|pos| map.get_key_value(pos).expect("pos").1[*next] ).collect::<Vec<&str>>();
        //dbg!(&current_pos);
        test_all = current_pos.iter().all(|pos| pos.ends_with('Z'));
        //dbg!(&current_pos);
        //current_pos = map.get_key_value(current_pos).expect("a direction").1[instruction];
        current_step += 1;
        //println!("curent pos {current_pos}");
    // }
    }
    //dbg!(current_pos);
    current_step.to_string()
}

fn build_map(instructions_split: &str) -> (HashMap<&str, [&str; 2]>, Vec<&str>) {
    let mut map = HashMap::new();
    let mut current_positions = vec![];
    for line in instructions_split.lines() {
        let key = &line[0..3];
        if key.ends_with('A') {
            current_positions.push(key);
        }
        let left = &line[7..10];
        let right = &line[12..15];
        map.insert(key, [left, right]);
    }
    //dbg!(&map);
    (map, current_positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let result = solve(include_str!("./input1.txt"));
        assert_eq!(result, "6");
    }

   #[test]
    fn solve_puzzle_input() {
        let result = solve("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");
        assert_eq!(result, "6");
    }

}
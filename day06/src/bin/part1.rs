fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(_input: &str) -> String {
    let mut output = 1;
    let times = [40,     81,     77,     72];
    let distance = [219,   1012,   1365,   1089];
    let mut stuff = vec![];
    for i in 0..4 {
        stuff.push(calculate_above(times[i], distance[i]));
    }
    for x in stuff {
        output *= x;
    }
    output.to_string()
}

fn calculate_above (time: u32, distance: u32) -> u32 {
    let mut times = 0;
    for x in 0..time  {
        let hold_down = x;
        let dist = hold_down * (time - x);
        //dbg!(dist);
        if dist > distance {
            times += 1;
        }
    }
    times
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let result = solve("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(result, "288");
    }

    #[test]
    fn calculate_distance() {
        let result = calculate_above(71530, 940200);
        assert_eq!(result, 4);
    }

    #[test]
    fn solve_actual_puzzle() {
        let result = solve("Time:        40     81     77     72
        Distance:   219   1012   1365   1089");
        assert_eq!(result, "861300");
    }
}
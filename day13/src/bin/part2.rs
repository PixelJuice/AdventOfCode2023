use itertools::{repeat_n, Itertools};

#[derive(Debug)]
struct Puzzle {
    lines: Vec<String>,
    collums: Vec<String>,
}

impl Puzzle {
    fn process(&mut self, input: &str) {
        let col_count = input.lines().next().expect("line").trim().len();
        self.collums.append(&mut repeat_n(String::new(), col_count).collect::<Vec<String>>());
        for line in input.lines() {
            self.lines.push(line.to_string());
            for (index, char) in line.chars().enumerate() {
                self.collums[index].push(char);
            }
        }
    }

    fn solve(&mut self) -> u32 {
        dbg!(&self.lines);
        dbg!(&self.collums);
        let options = self.lines.iter().enumerate().tuple_windows().filter(|((_, a), (_, b))| {
            let mut wrong_chars = 0;
            for (idx, a_char) in a.chars().enumerate() {
                if a_char != b.chars().nth(idx).unwrap() {
                    wrong_chars += 1;
                    if wrong_chars > 1 {
                        return false;
                    }
                }
            }
            return true;
        }).map(|((_, _), ( b, _))| b).collect::<Vec<usize>>();
        for opt in options {
            let score = opt * 100;
            let (left, right) = self.lines.split_at(opt);
            //let len = cmp::min(left.len(), right.len()) * 2;
            let mut wrong_chars = 0;
            if left.iter().rev().zip(right.iter()).all(|(a,b)| {
                for (idx, a_char) in a.chars().enumerate() {
                    if a_char != b.chars().nth(idx).unwrap() {
                        wrong_chars += 1;
                        if wrong_chars > 1 {
                            return false;
                        }
                    }
                }
                return true;
            }) {
                //println!("found horizontal");
                if wrong_chars == 1 {
                    return score.try_into().unwrap()
                } 
            }
        }
        
        let options = self.collums.iter().enumerate().tuple_windows().filter(|((_, a), (_, b))| {
            let mut wrong_chars = 0;
            for (idx, a_char) in a.chars().enumerate() {
                if a_char != b.chars().nth(idx).unwrap() {
                    wrong_chars += 1;
                    if wrong_chars > 1 {
                        return false;
                    }
                }
            }
            return true;
        }).map(|((_, _), ( b, _))| b).collect::<Vec<usize>>();
        for opt in options {
            let score = opt;
            let (left, right) = self.collums.split_at(opt);
            //let len = cmp::min(left.len(), right.len()) * 2;
            let mut wrong_chars = 0;
            if left.iter().rev().zip(right.iter()).all(|(a,b)| {
                for (idx, a_char) in a.chars().enumerate() {
                    if a_char != b.chars().nth(idx).unwrap() {
                        wrong_chars += 1;
                        if wrong_chars > 1 {
                            return false;
                        }
                    }
                }
                return true;
            }) {
                //println!("found vertical");
                if wrong_chars == 1 {
                    return score.try_into().unwrap()
                } 
            }
        }
        0
    }


}

fn main() {
    let input = include_str!("./input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let puzzle_inputs = input.split("\r\n\r\n").collect::<Vec<&str>>();
    let mut score = 0;
    for p in puzzle_inputs {
        //println!("start puzzle");
        let mut puzzle = Puzzle{ lines: vec![], collums: vec![] };
        puzzle.process(p);
        score += puzzle.solve();
    }
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test_first_puzzle() {
        let result = solve("..##...
##..###
#....#.
.#..#.#
.#..#.#
#....#.
##..###
..##...
#.##..#");
        assert_eq!(result, "3");
    }

    #[test]
    fn solve_test_second_puzzle() {
        let result = solve("#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#");
        assert_eq!(result, "100");
    }

    #[test]
    fn solve_test_puzzle() {
        let result = solve(include_str!("./test_input.txt"));
        assert_eq!(result, "400");
    }
     
}
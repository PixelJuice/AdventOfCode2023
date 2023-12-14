use itertools::repeat_n;

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
            //dbg!(&line);
            for (index, char) in line.chars().enumerate() {
                self.collums[index].push(char);
            }
        }
    }

    fn solve(&mut self) -> u32 {
        for i in 0..self.collums.len() {
            self.collums[i] = self.move_up(self.collums[i].clone());
        }
        //dbg!(&self.collums);
        let mut score = 0;
        for s in &self.collums {
            for (i, c) in s.chars().enumerate() {
                if c == 'O' {
                    score += 1 * (s.len() -i);
                }
            }
            //dbg!(&s);
            //dbg!(score);
        }
        score.try_into().unwrap()
    }

    fn move_up(&self, col: String) -> String {
        let mut moved = false;
        let mut new_string = col.clone();
        for (i, c) in col.chars().enumerate() {
            if i == 0 {
                continue;
            }
            if c == 'O' && col.chars().nth(i -1).unwrap() == '.' {
                new_string = self.swap(new_string, i, i-1);
                moved = true;
            }
        }
        if moved {
            return self.move_up(new_string);
        } else {
            return new_string;
        } 
    }

    fn swap(&self, s: String, from_idx: usize, to_idx: usize) -> String {
        let mut chars: Vec<_> = s.chars().collect();
        // swap the characters in the Vec
        chars.swap(from_idx, to_idx);
        // convert Vec back to String
        chars.into_iter().collect()
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
    fn solve_test_puzzle() {
        let result = solve(include_str!("./input_test.txt"));
        assert_eq!(result, "136");
    }
}
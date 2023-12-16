use itertools::repeat_n;

#[derive(Debug)]
struct Puzzle {
    lines: Vec<String>,
    collums: Vec<String>,
}

impl Puzzle {
    fn process(&mut self, input: &str) {
        let col_count = input.lines().next().expect("line").trim().len();
        self.lines = vec![];
        self.collums = repeat_n(String::new(), col_count).collect::<Vec<String>>();
        for line in input.lines() {
            self.lines.push(line.to_string());
            //dbg!(&line);
            for (index, char) in line.chars().rev().enumerate() {
                self.collums[index].push(char);
            }
        }
    }

    /*fn print(&mut self , input: &str) {
        let col_count = input.lines().next().expect("line").trim().len();
        self.lines = repeat_n(String::new(), col_count).collect::<Vec<String>>();
        for line in input.lines().rev() {
            //self.lines.push(line.to_string());
            //dbg!(&line);
            for (index, char) in line.chars().enumerate() {
                self.lines[index].push(char);
            }
        }
        dbg!(&self.lines);
        dbg!(&self.collums);
    }*/

    fn drop(&mut self) {
        for i in 0..self.collums.len() {
            self.collums[i] = self.move_up(self.collums[i].clone());
        }
    }

    fn score(&self) -> i32 {
        let mut score = 0;
        for s in &self.collums {
            for (i, c) in s.chars().enumerate() {
                if c == 'O' {
                    score += 1 * (s.len() -i);
                }
            }

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

    fn solve(&mut self) -> bool {
        //let mut copy = self.collums.clone();
        let copy = self.collums.clone();
        //dbg!(&self.collums); 
        for _ in 0..4 {
            self.drop(); 
            self.rotate_right(self.collums.join("\n").as_str());
        }
        if copy == self.collums {
            println!("solved");
            //dbg!(&self.collums);
            return true
        }
        false
    }

    fn rotate_right(&mut self, input: &str) {
        let col_count = input.lines().next().expect("line").trim().len();
        //self.lines = vec![];
        self.collums = repeat_n(String::new(), col_count).collect::<Vec<String>>();
        for line in input.lines().rev() {
            //self.lines.push(line.to_string());
            //dbg!(&line);
            for (index, char) in line.chars().enumerate() {
                self.collums[index].push(char);
            }
        }
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
        //while !puzzle.solve() { }
        for _ in 0..1000 {
            puzzle.solve();
            //puzzle.print( puzzle.collums.join("\n").as_str()) 
        }
        //dbg!(&puzzle.collums); 
        score += puzzle.score();
    }
    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_score() {
        let result = solve(include_str!("./input_test2.txt"));
        assert_eq!(result, "64");
    }
}
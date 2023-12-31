use std::cmp;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

#[derive(Debug)]
enum Content {
    Number(u32),
    Symbol(char),
    Blank
}



fn solve(input: &str) -> String {
    let mut grid: Vec<Vec<Content>> = vec![vec![]];
    let positions: [(i32, i32); 8] = [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ];
    let mut y = 0;
    for line in input.lines()  {
        let mut x = 0;
        grid.push(vec![]);
        for symbol in  line.chars() {
            
            if symbol.is_ascii_digit() {
                let mut temp_x = x;
                let mut add_one = true;
                let mut value:String = String::from("");
                loop { 
                    match line.chars().nth(temp_x) {
                        Some(c) => {
                            if c.is_ascii_digit() {
                                if symbol == '6' {
                                    println!("hello 6 {temp_x}")
                                }
                                if temp_x == 0 {
                                    add_one = false;
                                    break
                                } else {
                                    temp_x -= 1;
                                }
                            } else {
                                break;
                            }  
                        }
                        None => break,
                    }
                }
                if add_one {
                    temp_x += 1;
                }
                
                println!("start at {temp_x}");
                while line.chars().nth(temp_x).unwrap_or('.').is_ascii_alphanumeric() {
                    value.push(line.chars().nth(temp_x).unwrap());
                    temp_x += 1;
                }
                println!("{value} {x}:{y}");
                if !value.is_empty() {
                    grid[y].push(Content::Number(value.parse::<u32>().expect("Expect a number")));  
                }  
            }
            else if symbol == '.' {
                grid[y].push(Content::Blank);
            }
            else {
                grid[y].push(Content::Symbol(symbol));
            }
            
            x += 1;
        }
        y += 1;
        
    }
    let mut final_value: u32 = 0;
    let mut y: i32 = 0;
    
    for row in &grid {
        let mut x: i32 = 0;
        for symbol in row {
            let mut values:Vec<u32> = vec![];
            match symbol {
                Content::Number(_) => (),
                Content::Symbol(c) => {
                    println!("char is {c}");
                    for pos in positions {
                        let mut test_pos_x = 0;
                        let mut test_pos_y = 0;
                        if x != 0 {
                            test_pos_x = cmp::max((x + pos.0) as usize, 0);
                            test_pos_x = cmp::min(test_pos_x, grid[y as usize].len() -1);
                        }
                        if y != 0 {
                            test_pos_y = cmp::max((y + pos.1) as usize, 0);
                            test_pos_y = cmp::min(test_pos_y, grid.len() -1);
                        }

                        match grid[test_pos_y][test_pos_x] {
                            Content::Number(num) => {
                                println!("number added {num}");
                                values.push(num)
                            },
                            Content::Symbol(_) => (),
                            Content::Blank => (),
                        }
                    }
                },
                Content::Blank => (),
            }
            values.sort_unstable();
            values.dedup();
            for num in values {
                final_value += num;
            }
            x += 1;
        }
        y += 1
    }
    
    final_value.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let result = solve("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");
        assert_eq!(result, "4361");
    }
}
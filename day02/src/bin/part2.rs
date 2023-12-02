fn main() {
    let input = include_str!("./input2.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let colours = ["blue", "green", "red"];
    let mut return_value = 0;
    for line in input.lines() {
        let mut max_amount_shown: [u32; 3] = [0,0,0];
        let game = line.split_once(':').unwrap();
        game.1.split(split_on_chars).for_each(|shown_item| {
            for colour in 0..colours.len() {
                match shown_item.find(colours[colour]) {
                    Some(_) => {
                        let mut amount = String::new();
                        for letter in shown_item.trim().chars() {
                            if !letter.is_ascii_digit() {
                                break;
                            }
                            amount.push(letter);
                        }
                        let new_amount = amount.parse::<u32>().unwrap();
                        if new_amount >  max_amount_shown[colour]{
                            max_amount_shown[colour] = new_amount;
                        }
                    },
                    None => (),
                }
            }
            
        });
        let result = max_amount_shown[0] * max_amount_shown[1] * max_amount_shown[2]; 
        return_value += result;
    }
    return_value.to_string()
}

fn split_on_chars(c: char) -> bool {
    c == ',' || c == ';'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let result = solve("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "2286");
    }
}
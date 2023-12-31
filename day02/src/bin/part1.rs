fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let colours = ["blue", "green", "red"];
    let colour_max: [u32; 3] = [14, 13, 12];
    let mut return_value = 0;
    for line in input.lines() {
        let game = line.split_once(':').unwrap();
        let result = game.1.split(split_on_chars).all(|shown_item| {
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
                        if amount.parse::<u32>().unwrap() > colour_max[colour] {
                            return false
                        }
                    },
                    None => (),
                }
            }
            true
        });
        if result {
            let mut game_id = String::new();
            for letter in game.0[5..].chars() {
                if !letter.is_ascii_digit() {
                    break;
                }
                game_id.push(letter);
            }
            return_value += game_id.parse::<u32>().expect("this should be a number");
        }  
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
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
Game 100: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(result, "108");
    }
}
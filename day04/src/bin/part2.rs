use std::collections::HashSet;


#[derive(Debug, Clone)]
struct ScratchCard {
    card_nr:u32,
    winning_numbers:HashSet<u32>,
    ticket_numbers:HashSet<u32>,
}

impl ScratchCard {
    
}

fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let mut cards:Vec<ScratchCard> = vec![];
    for line in input.lines() {
        let first_split = line.split_once(':').unwrap();
        let card = first_split.0.split_ascii_whitespace().last().expect("card id").parse::<u32>().expect("number");
        let second_split = first_split.1.split_once('|').unwrap();
        let winning_numbers:HashSet<u32> = second_split.0.trim().split_ascii_whitespace().filter_map(|num| num.parse::<u32>().ok()).collect();
        let ticket_numbers:HashSet<u32> = second_split.1.trim().split_ascii_whitespace().filter_map(|num| num.parse::<u32>().ok()).collect();
        cards.push(ScratchCard { card_nr: card, winning_numbers: winning_numbers, ticket_numbers: ticket_numbers })
    }
    let num_cards = check_wins(&cards, &cards, cards.len() as u32);
    num_cards.to_string()
}

fn check_wins(cards: &Vec<ScratchCard>, original_cards: &Vec<ScratchCard>, mut total: u32) -> u32 {
    let mut new_cards = vec![];
    for card in cards {
        let card_number = card.card_nr as usize;
        let num_wins = card.winning_numbers.intersection(&card.ticket_numbers).count();
        new_cards.extend_from_slice(&original_cards[card_number..card_number+num_wins]);
    }
    total += new_cards.len() as u32;
    if new_cards.is_empty() {
        return total;
    }
    check_wins(&new_cards, original_cards, total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle() {
        let result = solve("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");
        assert_eq!(result, "30");
    }
}
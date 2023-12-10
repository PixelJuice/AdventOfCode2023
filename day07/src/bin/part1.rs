
use std::ops::Deref;

use itertools::Itertools;


fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,    
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    values: Vec<u32>,
    hand_type: HandType,
    bid: u32,
}

fn solve(input: &str) -> String {

    let mut something:Vec<Hand> = input.lines().map(|line| {
        let (cards, bid) = line.split_once(" ").expect("two &str");
        let hand:Vec<_> = cards.chars().map(|card| {
            match card {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                value => value.to_digit(10).expect("number smaller then 10")

            }
        }).collect();
        let bid_value = bid.parse::<u32>().expect("a number");
        (hand, bid_value)
    }).map(|(hand, bid)| {
        let values = hand.iter().counts().values().sorted().join("");
        let hand_typ = match values.deref() {
            "5" => HandType::FiveOfAKind,
            "14" => HandType::FourOfAKind,
            "23" => HandType::FullHouse,
            "113" => HandType::ThreeOfAKind,
            "122" => HandType::TwoPair,
            "1112" => HandType::OnePair,
            "11111" => HandType::HighCard,
            value => panic!("Should not exist {}", value),
        };
        Hand {
            values: hand,
            hand_type: hand_typ,
            bid: bid,
        }
    }).collect();

    something.sort_by(| a, b| match  a.hand_type.cmp(&b.hand_type){
        std::cmp::Ordering::Less => std::cmp::Ordering::Less,
        std::cmp::Ordering::Equal => {
            a.values.cmp(&b.values)
        },
        std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
    });
    //dbg!(&something);
    something.iter().enumerate().map(|(index, hand)| {
        println!("{0} * (1 + {index} at hand {1:?})", hand.bid, hand.values);
        hand.bid * (1 + index as u32)
    }).sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("32T3K 765", 765)]
    #[case("KK677 28", 28)]
    #[case("T55J5 684", 684)]
    fn solve_line(#[case] input: &str, #[case] result: u32) {
        let result_string = solve(input);
        assert_eq!(result_string, (result * 1).to_string());
    }

    #[test]
    fn solve_puzzle() {
        let result = solve("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");
        assert_eq!(result, "6440");
    }
}
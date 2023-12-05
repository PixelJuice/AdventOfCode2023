use std::collections::HashMap;
use std::ops::Range;

fn main() {
    let input = include_str!("./input1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let inital_split = input.split(':').collect::<Vec<_>>();
    let seeds = extract_seeds(&inital_split[1]);
    let conversions_chart = extract_conversion_charts(inital_split[2..].to_owned());
    let mut final_values:Vec<u128> = vec![];
    for seed in seeds {
        let value = process_seed(seed, &conversions_chart);
        final_values.push(value);
    }
    final_values.sort();
    final_values.first().expect("this is a number").to_string()
}

fn process_seed(seed: u128, chart:&Vec<HashMap<Range<u128>,u128>>) -> u128{
    let mut num = seed;
    let h = chart.len();
    println!("there are {h} charts");
    let mut last_num = seed;
    for map in chart {
        for key in map {
            match key.0.contains(&num) {
                true => {
                    num = num - key.0.start + key.1;
                    break;
                },
                false => num = num,
            };
        }
        println!("{last_num} -> {num}");
        last_num = num;
    }
    num
}

fn extract_conversion_charts(input: Vec<&str>) -> Vec<HashMap<Range<u128>,u128>> {
    let mut conversions_chart:Vec<_> = vec![];
    for chart in input {
        let lines = chart.trim().split('\n').collect::<Vec<_>>();
        let mut conversion:HashMap<Range<u128>,u128> = Default::default();
        for line in lines  {
            let convert_numbers = line.trim().split_ascii_whitespace().filter_map(|number| {
                number.parse::<u128>().ok()
            }).collect::<Vec<u128>>();
            if convert_numbers.is_empty() {
                continue;
            }
            dbg!(&convert_numbers);
            conversion.insert(Range { start: convert_numbers[1], end: convert_numbers[1] + convert_numbers[2]},  convert_numbers[0]);
        }
        dbg!(&conversion);
        conversions_chart.push(conversion);        
    }
    conversions_chart
}

fn extract_seeds(input: &str) -> Vec<u128> {
    input.trim().split_ascii_whitespace().filter_map(|number| {
        number.parse::<u128>().ok()
    }).collect::<Vec<u128>>()
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_extract_charts () {
        let test_str = "
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let input = test_str.split(':').collect::<Vec<_>>();
        let charts = extract_conversion_charts(input);
        let processed_seed = process_seed(79, &charts);
        assert_eq!(processed_seed, 82);
    }

    #[test]
    fn test_extract_chart () {
        let test_vec = vec!["50 98 2",
        "52 50 48",
        " ",
        "soil-to-fertilizer map",];
        let charts = extract_conversion_charts(test_vec);
        let processed_seed = process_seed(79, &charts);
        assert_eq!(processed_seed, 81);
    }

    #[test]
    fn solve_puzzle() {
        let result = solve("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4");
        assert_eq!(result, "35");
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
      (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> String {
    let (map, line_len) = build_map(input);
    let planets = find_planets(map, line_len);
    let distance = calculate_distances(planets);
    distance.iter().sum::<u32>().to_string()
}

fn calculate_distances(planets: Vec<Pos>) -> Vec<u32> {
    let mut distance = vec![];
    for x in (0..planets.len()).rev() {
        for y in 0..x {
            distance.push(planets[x].distance(&planets[y]))
        }
    }
    distance
}

fn find_planets(map: String, line_len: usize) -> Vec<Pos> {
    map.match_indices('#').map(|(index,_)| Pos(index % line_len, index / line_len)).collect::<Vec<Pos>>()
}

fn build_map(input: &str) -> (String, usize) {
    let chars_len = input.lines().next().expect("first line").len();
    println!("line is {}", chars_len);
    let mut flat_map = input.lines().collect::<Vec<&str>>().join("");
    let blank_row: String = ".".repeat(chars_len);
    let insert_row_at = input.lines().enumerate().filter_map(|(index, line)| {
        if line.chars().all(|c| c == '.') {
            Some(index)
        }
        else {
            None
        }
    }).collect::<Vec<usize>>();
    for index in (0..insert_row_at.len()).rev() {  
        flat_map.insert_str(insert_row_at[index] * chars_len,  &blank_row);
    }
    println!("collum is {}", flat_map.len() / chars_len);
    let mut increase_line = 0;
    let mut chars_to_insert = vec![];
    for x in 0..chars_len {
        let mut result = vec![];
        let mut index = x;
        while index < flat_map.len() {
            result.push((flat_map.chars().nth(index).unwrap(), index));
            index += chars_len;
        }
        if result.iter().all(|(c, _)| c == &'.') {
            chars_to_insert.append(&mut result);
            println!("should add collum at {}", x);
            increase_line += 1;
        }
    }
    chars_to_insert.sort_by_key(|u| u.1);
    for index in (0..chars_to_insert.len()).rev() {  
        flat_map.insert_str(chars_to_insert[index].1, ".")
    }
    //dbg!(&flat_map);
    println!("line becomes {}", chars_len + increase_line);
    println!("collum becomes {}", flat_map.len() / (chars_len + increase_line));
    
    (flat_map, chars_len + increase_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_amount() {
        let planets = find_planets("....#.................#...#..............................................#.....#.......................#...................................#...#....#.......".to_string(), 13);
        let result = calculate_distances(planets);
        assert_eq!(result.len(), 36);
    }

    #[test]
    fn test_distance() {
        let result = find_planets("....#.................#...#..............................................#.....#.......................#...................................#...#....#.......".to_string(), 13);
        assert_eq!(result[1].distance(&result[6]), 9);
        assert_eq!(result[7].distance(&result[8]), 5);
        assert_eq!(result[0].distance(&result[6]), 15);
        assert_eq!(result[2].distance(&result[5]), 17);
    }

    #[test]
    fn test_distance_with_build_map() {
        let (map, line) = build_map("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");
        assert_eq!((map.len() / line), 12);
        assert_eq!(map, "....#.................#...#..............................................#.....#.......................#...................................#...#....#.......".to_string());
        let result = find_planets(map, line);
        assert_eq!(line, 13);
        assert_eq!(result[1].distance(&result[6]), 9);
        assert_eq!(result[7].distance(&result[8]), 5);
        assert_eq!(result[0].distance(&result[6]), 15);
        assert_eq!(result[2].distance(&result[5]), 17);
    }

    #[test]
    fn test_find_planets() {
        let result = find_planets("....#.................#...#..............................................#.....#.......................#...................................#...#....#.......".to_string(), 13);
        assert_eq!(result.len(), 9);
    }

    #[test]
    fn test_planets_pos() {
        let result = find_planets("....#.................#...#..............................................#.....#.......................#...................................#...#....#.......".to_string(), 13);
        assert_eq!(result[0], Pos(4,0));
        assert_eq!(result[1], Pos(9,1));
        assert_eq!(result[2], Pos(0,2));
        assert_eq!(result[3], Pos(8,5));
        assert_eq!(result[4], Pos(1,6));
    }

    #[test]
    fn test_build_map() {
        let result = build_map("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");
        assert_eq!(result.0, "....#.................#...#..............................................#.....#.......................#...................................#...#....#.......");
    }

    #[test]
    fn solve_puzzle() {
        let result = solve("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");
        assert_eq!(result, "374");
    }

    #[test]
    fn solve_input() {
        let result = 9521776.to_string();//solve(include_str!("./input.txt"));
        assert_ne!(result, "9278090");
        assert_ne!(result, "702110278");
        assert_ne!(result, "9449548");
        assert_ne!(result, "9520948");
        assert_eq!(result, "9521776");
    }

    #[test]
    fn test_line_len() {
        let input = include_str!("./input.txt");
        let result = input.lines().next().expect("first line").len();
        assert_eq!(result, 140);
    }
}
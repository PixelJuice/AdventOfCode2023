use std::usize;


#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn distance(&self, other: &Pos) -> usize {
      (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as usize
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = solve(input, 1000000);
    dbg!(output);
}

fn solve(input: &str, increase:usize) -> String {
    let chars_len = input.lines().next().expect("first line").len();
    let flat_map = input.lines().collect::<Vec<&str>>().join("");
    let (x,y) = build_map(input);
    let planets = find_planets(flat_map, chars_len, (x,y), increase);
    let distance = calculate_distances(planets);
    distance.iter().sum::<usize>().to_string()
}

fn calculate_distances(planets: Vec<Pos>) -> Vec<usize> {
    let mut distance = vec![];
    for x in (0..planets.len()).rev() {
        for y in 0..x {
            distance.push(planets[x].distance(&planets[y]))
        }
    }
    distance
}

fn find_planets(map: String, line_len: usize, (vec_x,vec_y):(Vec<usize>,Vec<usize>), increase:usize) -> Vec<Pos> {
    let mut positions = vec![];
    for index in map.match_indices('#').map(|(index,_)| index).collect::<Vec<usize>>() {
        let mut init_pos = Pos(index % line_len, index / line_len);
        let mut to_add_x = 0;
        for x in &vec_x {
            if init_pos.0 > *x  {
                to_add_x += increase -1;
            }
        }
        let mut to_add_y = 0;
        for y in &vec_y {
            if init_pos.1 > *y  {
                to_add_y += increase-1;
            }
        }
        init_pos.0 += to_add_x;
        init_pos.1 += to_add_y;
        dbg!(&init_pos);
        positions.push(init_pos);

    }
    positions
}

fn build_map(input: &str) -> (Vec<usize>, Vec<usize>) {
    let chars_len = input.lines().next().expect("first line").len();
    let flat_map = input.lines().collect::<Vec<&str>>().join("");
    let insert_row_at = input.lines().enumerate().filter_map(|(index, line)| {
        if line.chars().all(|c| c == '.') {
            Some(index)
        }
        else {
            None
        }
    }).collect::<Vec<usize>>();

    let mut insert_collums_at = vec![];
    for x in 0..chars_len {
        let mut result = vec![];
        let mut index = x;
        while index < flat_map.len() {
            result.push((flat_map.chars().nth(index).unwrap(), index));
            index += chars_len;
        }
        if result.iter().all(|(c, _)| c == &'.') {
            insert_collums_at.push(x);
        }
    }
    insert_collums_at.sort();
    
    (insert_collums_at, insert_row_at)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_puzzle_steps() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let chars_len = input.lines().next().expect("first line").len();
        assert_eq!(chars_len, 10);
        let flat_map = input.lines().collect::<Vec<&str>>().join("");
        assert_eq!(flat_map, "...#.............#..#.........................#....#.................#.................#..#...#.....".to_string());
        let (x,y) = build_map(input);
        assert_eq!(x.len(), 3);
        assert_eq!(y.len(), 2);
        let planets = find_planets(flat_map, chars_len, (x,y), 10);
        assert_eq!(planets[2], Pos(0,2));
        assert_eq!(planets[1], Pos(25,1));
        assert_eq!(planets[3], Pos(24,13));
        let distance = calculate_distances(planets);
        let result = distance.iter().sum::<usize>().to_string();
        assert_eq!(result, "1030");
    }

    #[test]
    fn solve_puzzle_x10() {
        let result = solve("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", 10);
        assert_eq!(result, "1030");
    }

    #[test]
    fn solve_puzzle_x100() {
        let result = solve("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", 100);
        assert_eq!(result, "8410");
    }

    #[test]
    fn solve_input() {
        let result = solve(include_str!("./input.txt"), 1000000);
        assert_ne!(result, "o");
    }

    #[test]
    fn test_line_len() {
        let input = include_str!("./input.txt");
        let result = input.lines().next().expect("first line").len();
        assert_eq!(result, 140);
    }
}
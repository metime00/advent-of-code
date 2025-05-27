// https://adventofcode.com/2024/day/4
use itertools::{self, Itertools};

// no (0,0)
const SLOPES: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const PATTERN: [char; 4] = ['X', 'M', 'A', 'S'];

type Grid = Vec<Vec<char>>;

// fn process_input(input: &str) -> Grid {

// }

// First pass finds all X's. Second pass on all X's finds all directions with M, and continues down those directions for a and s.
fn solve_simple(input: &str) -> i32 {
    -1
}

// fn solve_complex(input: &str) -> i32 {}

pub fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // Puzzle tests
    // #[test]
    // fn solve_simple_input_1() {
    //     assert_eq!(solve_simple(INPUT_1), 161);
    // }

    // #[test]
    // fn solve_simple_input_2() {
    //     assert_eq!(solve_simple(INPUT_2), 159833790);
    // }

    // #[test]
    // fn solve_complex_input_3() {
    //     assert_eq!(solve_complex(INPUT_3), 48);
    // }

    // #[test]
    // fn solve_complex_input_2() {
    //     assert_eq!(solve_complex(INPUT_2), 89349241);
    // }
}

const INPUT_1: &str = r#"
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

const INPUT_2: &str = r#""#;

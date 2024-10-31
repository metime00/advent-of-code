// https://adventofcode.com/2023/day/14
//
// Each column is independent, so the first breakdown for evaluation should be by column.
// First idea is to walk downward, checking for stop points (either a # or the top edge), and adding the value of O rocks that are at their final position.
// Walk downward, counting O's until reaching a free space (i). Upon reaching a free space, scan downward until finding either a O or a #.
//      If a O is found, move it to the free space, add its value to the total count, and start again from (i+1).
//      If a # is found at position j, start again from (j+1)

use core::fmt;
use std::{time::Instant, usize};

pub fn main() {
    println!("example load: {}", simple_solve(INPUT_1));
    println!("problem load: {}", simple_solve(INPUT_2));

    println!("second half!");
    println!("example load: {}", solve_complex(INPUT_1)); // this is broken due to bad instructions in part 2
    println!("problem load: {}", solve_complex(INPUT_2));
}

pub fn simple_solve(input: &str) -> usize {
    let input = process_input(input);
    calculate_board_load(&roll_board(&input))
}

fn solve_complex(input: &str) -> usize {
    let input = &process_input(input); // one first roll in the original north orientation.
    let mut working_board = input.clone();
    let now = Instant::now();

    let mut board_states = Vec::new();
    // board_states.push(working_board.clone()); // put in the initial state.
    for _i in 0..1000000000 {
        if _i % 1000000 == 0 {
            println!("iteration: {}", _i);
            println!("time elapsed: {}", now.elapsed().as_secs()); // Test took about 33 seconds per million cycles, which means to do a billion cycles would be 9-10h.
        }
        for _j in 0..4 {
            // roll, then rotate, so each cycle ends with a rolled eastward board in a north orientation.
            working_board = rotate_board(&roll_board(&working_board));
        }

        // println!("iteration: {}", _i);
        // println!("board load: {}", calculate_board_load(&working_board));
        // print_board(&rotate_board(&working_board));

        // Check for cycles and add current board
        if board_states.contains(&working_board) {
            // cycle detected because a duplicate board state has been found. This means we can calculate the billionth board state with a modulus.
            println!(
                "cycled detected that is {} iterations long.",
                board_states.len()
            );
            break;
        }
        board_states.push(working_board.clone());
    }
    println!(
        "the billionth cycle is iteration: {}",
        1000000000 % board_states.len()
    );
    working_board = board_states[1000000000 % board_states.len()].clone();
    calculate_board_load(&working_board)
}

fn calculate_board_load(input: &Board) -> usize {
    input
        .0
        .iter()
        .fold(0, |load, i| load + calculate_row_load(i))
}

// calculate load on a fully rolled row.
fn calculate_row_load(input: &Vec<Space>) -> usize {
    let mut load = 0;
    for i in 0..input.len() {
        match input[i] {
            Space::Stone => {
                load = load + (input.len() - i);
            }
            _ => (),
        }
    }
    load
}

fn roll_board(input: &Board) -> Board {
    Board(input.0.iter().map(roll_row).collect())
}

fn roll_row(input: &Vec<Space>) -> Vec<Space> {
    let width = input.len();
    let mut output: Vec<Space> = vec![Space::Empty; width];
    let mut cur_destination: usize = 0;
    for i in 0..width {
        match input[i] {
            Space::Stone => {
                // simulate stone rolling by removing stone from current space, and placing it in destination space.
                // this will retain stone position if it was already in a fully rolled place.
                output[i] = Space::Empty;
                output[cur_destination] = Space::Stone;
                cur_destination = cur_destination + 1;
            }
            Space::Wall => {
                output[i] = Space::Wall;
                cur_destination = i + 1;
            }
            Space::Empty => {
                output[i] = Space::Empty;
            }
        }
    }
    output
}

// rotates the input board counter-clockwise.
fn rotate_board(input: &Board) -> Board {
    let width = input.0.len();
    let mut output: Board = Board(vec![vec![Space::Empty; width]; width]);
    for i in 0..width {
        for j in 0..width {
            output.0[j][(width - 1) - i] = input.0[i][j];
        }
    }
    output
}

// Reads in input with a north->south row orientation for the 2d vector.
fn process_input(input: &str) -> Board {
    let input = input.trim();
    let width = input.lines().next().unwrap().trim().len();
    let mut output: Board = Board(vec![vec![Space::Empty; width]; width]);
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.len() != width {
            panic!("line {} doesn't match width.", line.len());
        }
        for (j, ichar) in line.chars().enumerate() {
            let space = match ichar {
                'O' => Space::Stone,
                '#' => Space::Wall,
                '.' => Space::Empty,
                e => panic!("invalid input: {}", e),
            };
            output.0[(width - 1) - j][i] = space;
        }
    }
    output
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Board(Vec<Vec<Space>>);

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = "".to_string();
        for i in &self.0 {
            let mut line = "".to_string();
            for j in i {
                line = line + &j.to_string();
            }
            output = output + &line + "\n";
        }
        f.write_str(&output)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Space {
    Wall,
    Stone,
    Empty,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Space::Stone => f.write_str("O"),
            Space::Wall => f.write_str("#"),
            Space::Empty => f.write_str("."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn solve_row(input: &Vec<Space>) -> usize {
        calculate_row_load(&roll_row(&input))
    }

    #[test]
    fn input_1_simple() {
        assert_eq!(simple_solve(INPUT_1), 136);
    }

    #[test]
    fn input_2_simple() {
        assert_eq!(simple_solve(INPUT_2), 106378);
    }

    #[test]
    fn input_2_complex() {
        assert_eq!(solve_complex(INPUT_2), 90795);
    }

    #[test]
    fn single_column_count_static_stone() {
        let input1 = [Space::Stone, Space::Empty].to_vec();
        assert_eq!(solve_row(&input1), 2);

        let input2 = [Space::Stone, Space::Stone].to_vec();
        assert_eq!(solve_row(&input2), 3);
    }

    #[test]
    fn single_column_stone_should_fall() {
        let input1 = [Space::Empty, Space::Stone, Space::Empty].to_vec();
        assert_eq!(solve_row(&input1), 3);

        let input2 = [Space::Empty, Space::Stone, Space::Stone].to_vec();
        assert_eq!(solve_row(&input2), 5);
    }

    #[test]
    fn single_column_respect_walls() {
        let input = [
            Space::Empty,
            Space::Stone,
            Space::Wall,
            Space::Empty,
            Space::Stone,
        ]
        .to_vec();
        assert_eq!(solve_row(&input), 7);
    }

    #[test]
    fn rotate_once() {
        let input = Board(
            [[Space::Empty, Space::Stone], [Space::Empty, Space::Empty]]
                .map(|x| x.to_vec())
                .to_vec(),
        );
        let expected = Board(
            [[Space::Empty, Space::Empty], [Space::Empty, Space::Stone]]
                .map(|x| x.to_vec())
                .to_vec(),
        );
        assert_eq!(rotate_board(&input), expected);
    }

    #[test]
    fn rotate_twice() {
        let input = Board(
            [[Space::Empty, Space::Stone], [Space::Empty, Space::Empty]]
                .map(|x| x.to_vec())
                .to_vec(),
        );
        let expected = Board(
            [[Space::Empty, Space::Empty], [Space::Stone, Space::Empty]]
                .map(|x| x.to_vec())
                .to_vec(),
        );
        assert_eq!(rotate_board(&rotate_board(&input)), expected);
    }

    #[test]
    fn rotate_thrice() {
        let input = Board(
            [[Space::Empty, Space::Stone], [Space::Empty, Space::Empty]]
                .map(|x| x.to_vec())
                .to_vec(),
        );
        let expected = Board(
            [[Space::Stone, Space::Empty], [Space::Empty, Space::Empty]]
                .map(|x| x.to_vec())
                .to_vec(),
        );
        assert_eq!(rotate_board(&rotate_board(&rotate_board(&input))), expected);
    }

    #[test]
    fn rotate_fource() {
        let input = Board(
            [[Space::Empty, Space::Stone], [Space::Empty, Space::Empty]]
                .map(|x| x.to_vec())
                .to_vec(),
        );
        assert_eq!(
            rotate_board(&rotate_board(&rotate_board(&rotate_board(&input.clone())))),
            input
        );
    }

    #[test]
    fn solve_complex_simple_case() {
        let input = r#"
.#...
.#O..
.O.O.
...#.
.#...
        "#;

        assert_eq!(solve_complex(input), 5);
    }
}

const INPUT_1: &str = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;

const INPUT_2: &str = r#"
O....##.##.....OO...O.O......#.OO.#.#....#.#..#..##....###..O.....O#..#.#.......O.#.##...O#OO.O....O
.#..OOO..#.#..O..O...O.#.......O..O....##..O##O..#..#.......O.O.#....O...O#.O#OOO...O...#O.#......O.
..#O...#O.......##.O.O..O#..OO..O...#.O...O.#...#.OOO.OO.O......#.....#.O..##.O.#.#...O.O#..#..#O.#.
O####.O.#.O...#..#...O.......#....#..O....OO#...O..##...#..##..O..####..OO.O..O.#OO..O.O..O##O.....#
OO..O.##..#.O##O.....#..#O#.O...#.......#O#...#.O....#..O.O.....O#.OO....O......O#O.#..O..OO.O.O....
..#O.O...O...O..O.OO.O#OO...#......#.....O.#.....#.....O##.#......#O..OO.......#..#..O...#.......O.O
.O...#.......OO..O.....#....#.#.O.........#....#......#.OO...O.........O.O.O....O..#..#O..##...#.O..
O#......#.O....#.O.#....#....O.......#O.O#O..O..#...#.O.#..O.OO...O..O....#O#OOOO................#..
O..O.#..#........O......O..O..#..O.#.O...#O...O##..O.....#OO.#..#...........##.#...O##.O...#..#..O..
..O.....OO.O#..#O#.....#......OOOO#..#.O.#.#..O..O.....O.O.#....#O...O...O.OO......#O#.#.##......O.O
.#.#............OO...O.OOO#O..#.#O#.O.#O#.O.O.....#.O..........#.....O#..O...O#.....O......##O..#OO.
.O#O....O#..#....OO.....#...O...O..O.O..O..##....#.#.....O....O..#O..........#O...#O##.#....O..##O..
....OOO.O..O#..#.OO.....O#..O.O#...#.....##OO.....OO...##O.....O......O#..O##...#...##.O..O....#.#..
.#......O.#..O...........O...#..O#.O.#O#..OO.........O.O...O##.........##.O#.........#.O......#...#.
O.#...##..#..........#OO........O#..##..##.#O#O..O..#O..O....O.OO....O...OOO...O.....#.OO#..#.#.....
.##.#O.##..#....O.##O..O.#..O...#...O....O#....O...O.#O#...........##O..O..#.O....O........O.#...OOO
.O#......OOOO...#..O....#..#.O#.#.##.##O#.#.#..O....O#..........O..#..O...#.....O........#....O.#..#
.....O......#..#...O....OO...O#OO...OO.O....#..O.O.....O#OO.#....#.#.#......O......OO........#.#.##.
......O......O...O##.#O........O.......#.##.O......O.#.#.#.....#.O....#..#O...O##..#..#.##...O#O.O.O
..OO.O...#..OO#.OO..O......##......O.#...#....O.OO..O...#.......#O##..OO....OO.O..#.O...#O..#.#O....
O...O.#.O..#.OOO...........O..#.#....OO..OO.O....O#O#O#..OOOO....OOO.OOO#....#....O#..O#.O.#....O...
.......O.....O..O.#...O.#...#O.O..#......#...O#OOOO#..OOO..#..O....#...O..O..O.#....O.O.O.O...#O....
#...O.#.........OO...#......OO....O.##OOO#O.......#.O..#..O.#O#.O...O.OOO.O##OO.#.O.O..#.....O...O#.
O.OOOOO##O#.......O#.....O...O.#...O..OOO.....##...O...O....O.#....##O....#.#OO..#..#O..OO#OO#......
...........O...#.O....O##O..O.O....#.#O...O..OO.......#..#.#...#.........##..#.O...#.O...OOO..O.#OO.
..O##....#....O.#.O#O##.OOOOO..O.#..#...O..#...#.##.#O.....#..O..O.O#...#.O..#....#O.#........O#...#
#...O.......#.O....O......OO#.##..##O..O..O....O.O##.OOO......#......#...OO..O.O..........OO.O.O..O.
........OO.O.#.#.O...............O......O...O....#........O..O....O..O.....O.O..O.O.O#O#.O..##......
O..O.......#O..O..O.O....O......O##.#...O##..#...O.O#.#.O..O.O#.#.....O...O#.#.#.....O.....#.#......
.#......O..O..#O....O...O...O.......#..#..#.O...##..#......O.O.OOOO...##..#.OO......#O.O.O#...O.##O#
..OOO...#.O.O..#.#..O....##.O....#.O..OO#.O.......O..##O##.#OO.....#.....#.....O.#O.OO#....#O..OO.O.
..O#OO#.#....O.#..O.#OO#.#...O.#.#.OO....O..O.#...#..O..OO.#.O#..##.#......#O.#.#..O..O...OO#..OOO..
#.#OO.....O...O......O............O...###..O.#.OO...O...#...O.....##OO#.......#...#.OO.#...#....#...
....O....#.#OO##OO...OO...#..#.#.....O.....O..O....#......O....O#....OO.#....O....O...#.O..#OOO.OOOO
.#..O.O.O.O.#.O##O..#..O...........O..........OO#.#..O...#O....#OO...O.#.....O.OOOO....O.......#O.#.
...O##....O.....#O..#.OO........#...O..#O#..OO#........###.O..O......O..#...##.#..#..#O.....O.#..#.#
.#..O.....O...OO#OO..#......O.#....#..#.#.....O.....OO.O...#.....###...O....O.......OO#..#....O.#..O
O...#.O..O....#.O....#...#.O....#.O..##.O.O#.O#O.O.....O.#OO#.#O......#.O..#.#..#........O.O...O..O#
......O.OOOO.O....O.#.##.O...#O....OOO...O..O###..OOO.........OO.O..O...OO.##..#.#..##.O.#...#....#.
.#O...#.##.O....#.#...##O....O#O.O.#.O.#.O...#...O.#.O#O....O.#..###.O.#OOOO.O#OO.....OOO#.....##..O
..O##O.#.#..O...#O..O.##.#O..O.#..OO.O..........O.....O#.#.OOO........#O.O#.#..O..OOO....#..O.O.#..O
OO.#O....OO#........O.O.#..#..O#.O.OO#O...O.#.O........O##.OO..O..#..O........O#.#O.............O...
................O#...........O........O#.O..O#..#O.O.O..#.....#.#O..O.O.#.....#.O.....#...#.#O..#..#
#....#...#....#.#O......##O.....#.#........#.O....OO#..OO.#....OO..#....O....OO..#..##......#O.O.#..
O####O.##....####O..#.#.OO.O..#O#O...O.OOO...#O##..##......O...O..OO..#.O#O...O.#O........O.....#..#
...OOO.O.....O..#.O.....O.OO...OOO#O.........O...O..#...#OO.....#..O....#....#.......##.....#...#.O#
#.....O#..O#.#.....#..O......#..O.......O##.....##.##O..OO##.OO#..##......O......O....O....O#....O..
O....OO......O.....O...O#.O..#.#...#....#..#....OO.O##O..##O..O#.O.#.OO#.OOO....#.OO..#O..O..#..##..
O.O#.#OO..O..O...OO......OO.......#.O#O.O#....#.O#O..O#.#...O#..O...O....O.#OO#.#..O.##..O.....#..OO
.##.O...O.O..O....OO...O#....OO..#.....#.....OO##.#.O.#......#.O...O..O...##..#O.....#.O....#O......
.OO#..O.O.....#.O.OO.##..#.....O......O..OO...O.O.O##OO.#O...O..O..O#...OO..O.....#.............O.OO
..O....#O..##.O..#...O#..#.OO....##.O.....#..##.O...#.O...O.#O.OO..#O...O##.O.......#.O.#.O..#O.....
.##..........O..#.#.........O....O..O#..##.#..#..O#OO..#..#O...O#O.#..#.#.O.OO......O###.....O.O....
...#O..O..O#..O#..O#...OO.O.O.#.OO..#.#..#.#.......O.O..#OO#..O....O...#O#..#O....O.#O.#..#.#..O...#
OO###....OO..#.##OO......#..O###..O.....O.O.#...OO..#O....OO..#O..#.#..OO.#.....O......OOO........OO
.O....#..#.O.O........O..#....O...O...O#.....OOOO.####.O..#.##.##O##..O.OO...O.OO..#O...O..O.O.#.O..
##O#...#...O.O#..O..O.#.OOO..O..##....#....##.O.#.O....O.#....O##...OO..O..#..O.O.#.....#OOO.O..O...
.O.....#....O#.O......#O...#.#.#...O.#..O.O..#...O.#O..O..O......#...........O##..O#O.#.#.#.........
OO.....O..#.O#.....##O..##....OO....O..O.O..#..O.O........O...#...O....O#O.O..#.O..##.#...#.....###.
.#..........O.....#O#O....O.OO.#..#...#..O.#...O.#.#.O#O....#O.O..#.....##O#.#.O..#O....O.##...#.O.O
.O....O....O##.O...O....O....O....O..O............#O...#.#...OOO....O#.....O#...##.......O.#........
.#.......OOO#............OO..........O.#...O..OO....#..O.#...##.O#....O..O.#.O#O##.O.O...#.#O#.O.O..
.#..#.#...........O.#..##..OO#.O..O..O.#OO##O.O..OOO....OO..#OO..#..###..O....##...........#O.###.O.
O....O..O...#........O.#OOOOO#.O.....O.#...O.OOO#.#......#.OO#.#.O..#..#O....##....#.###...#.###..#.
..O...O#.O.....#.OOO.O....O.......#O...O..#.#......#.#..O......OOO.#..O.#O..O..O#..#..#...#....O.#.O
O....O...OO.....#.....O#..#..##...O..O.....##....#O.....##.#..O#.O...#O.....O##.##O.....#O..O..O...#
O.##.OO.O.O.O...O..O..O...O.O...O.#..O...###....O..O##O#....OO.......#....#OO....#.#.......O....OO#O
.O......OO.....##...##.#..#..........##O..#.O#.#....#...#.OO..O.....#...O.......##.OO##....O..OO....
O....O#.....#.OO..O#..OO.OO#..O...O.O.OO.O.O.#...O.#OO..O#.....#.#.....#..O....O...........O#.#.#...
#..#.#..#.....#O.O.#####........O..#......#O......##.........#....#.#O.#.O...O.#..O.O..#O.#O..OO..#O
...O..O...O.OOOO.O#O###..O...#..#.......O#......O..O#O.OO.#..#.O.O...O....O.....###...#......##....O
..#.#O........O#O..##OOO....O##.#..O...#.O.O##..#OO.....#OO.#......O...#...#.O.#.O#.....O...O...##.#
O.....OO....OO#..O.....O....O..#.#.......OO#O........##.O..O#O.#.##.O..O.#.#.#......#O.##....O.#OO.O
..O..OO..O..O#....O...#....OO#O.......O....O..##O.O.O#..#O#....#..O.....#O.#.O#....##....O......#...
......#O#..O#..OO#...#......#.#.......O#..#.O#.O.O.OO...#..#O..##..#O..O...#..#.O....##O.O##O..O.#.O
..#O...#...O.........OO........O.O.O#......OO.#.....O.O..O#.#O#.....####.OO....#O#OO..#O.O..O##O...#
.#.O..#...O.#.O.OO###.#.OO#..O.#OO.#.#.#....O...#O#..OOOO#.......#.O...OO.....O#.#...##.............
.#.#.OO....O.........O..##O.#.O........O.#O.O##...O....O.O.#OOO....OO.O..O.OO...O..#OO...#......#..#
......O.....O..O...#...#......O..#O#...#...#...O...OO.........O..O.O.OOO.O.#.O.O....OO.O.#O..O...O..
........#....OO......#.....#O#...#..O.O..#..O..O.##O.O..........#...#.O...O#.O..........OOOO.O....O.
O..#.O.O....OO...#.O.......O....#.O#...O#.......OO..#O###O#O.O.O.#...#...#O#.O..O...OOO.OOO.#.#.....
.#.OO......O#.#...OO.#..O....O#O.O#..........O......#.#.O....O...O.#O.#OO........#..#...##..#....OOO
O.....#....O......O......OO#O#.##.##OO..O............O.OO...O#.....#...##...#...OO.OO.......O.....O.
.....OO.O......O.........O.#.O.....#.#O......O..O..#.#.##.........O##OO....O..OO...#........#...##..
#..###......O.....O#OO...#..#O....O.OO..O...........#..#.#...O##O...O.O....#.OO..#.#.OO....#..#O.O.#
.........O#.O#...##.#.#...O#.O.........O.#..OO........O.O.....#O....O#.#..O.O..#....#.....#.......O.
...#O...O.#...OOOO.#O..O.O#.....#.#.....O..#.....O.#.O......O.##O.O..O........#.........#O..O#..#...
#.#.#O#.O..#O.#O..#.##..O#.O..#.O.OOO...........O..O.O.#O..#....O....#O.##O.##O......O..OOO...#O...O
O.#O##.....#.O.O.OO.O..O....O...#O...........O.##O##...OO...##.#.#.#.#.O..O...#......O...O.#..#.#.O.
.OO....O.....#O.O........O#.......O...O.O##O..#....O.#O.#.O.....##..#...#.OO.O.O..##...#O.OOO#O..#.#
##O#OOO.##...#.O...#....#...O#.....O....#.OOO.O.##..OO.OO...O.O.O..O.O..#.#.#.##...O...#..#.#.O.....
..O#..#OOO#....O...#.O.#...#....#...#....O..........O.O#.....O.....O.OO...O.#.......O.#...O.........
#..#.#.O#.OO.O...O..#....#.O..OO.....#..O...O.#.#...OO....O#..O....###...###..O.O.O.O.O..OOO.....OO.
#..#..#.O..#..O.O..O#....##O.OO.O#OO.......O........#..O..#.O..#.#OOOO...O..#OOO....#..#..OOO......O
.#O.#.#.#...#OOO.OO..#O.##.....O..#O.##.O.O.O..OOO#....O.##......OOOOOO...O..O......#...#.O...O.#...
#O..O.O###...##O...O........##.O#....#O..O.O.O#.#OO##O.O.O#.#.##O.OO........O.........###O.O.#O#OO#O
O..#O......#.#O...O#...#####..#O#...O.##OO......O.....O.......O.O.#..O.O...........#OO..#..OOO....#.
....O.O#..#O#.#....#O.O##..#...O...#O.#.#.##O...OO....O..#..OO#........#O.........#O..O.#..OO.......
#..##.#.O#O..#O..OO...#.....#..O#..O....O#.....OO.O......O..#....O.##O#O...O....#..O.......O....O.##
#....#O..#O.O..#....OO#.#OO...O..O.......O..#......#.......#.O.#..OO..O........OO.......#O.O.#O....O
"#;

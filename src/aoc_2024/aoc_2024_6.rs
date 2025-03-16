// https://adventofcode.com/2024/day/6

use std::collections::HashSet;

use itertools::Itertools;

type Grid = Vec<Vec<char>>; // [row][column]
type X = isize; // where leftmost char is x = 0.
type Y = isize; // where top row is y = 0.
#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
struct Coord(X, Y);

// Grid + walk direction, that outputs (x,y) coordinates until an obstacle is reached, where it outputs None.
struct Griderator<'a> {
    grid: &'a Grid,
    position: Coord,
    direction_index: usize,
}

// The walking directions a guard. Up, right, down, left
const DIRECTION: [Coord; 4] = [Coord(0, -1), Coord(1, 0), Coord(0, 1), Coord(-1, 0)];

// returns the Coord as usize if valid, else None.
fn try_usize(coord: Coord) -> Option<(usize, usize)> {
    let Ok(row): Result<usize, _> = coord.1.try_into() else {
        return None;
    };
    let Ok(column): Result<usize, _> = coord.0.try_into() else {
        return None;
    };
    Some((column, row))
}

// whether the guard will still be on map next step.
fn on_map(coord: Coord, grid: &Grid, direction_index: usize) -> bool {
    match try_usize(Coord(
        coord.0 + DIRECTION[direction_index].0,
        coord.1 + DIRECTION[direction_index].1,
    )) {
        Some((column, row)) => row < grid.len() && column < grid[row].len(),
        None => false,
    }
}

impl<'a> Iterator for Griderator<'a> {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        let Some((column, row)) = try_usize(self.position) else {
            return None;
        };
        if row >= self.grid.len() || column >= self.grid[row].len() || self.grid[row][column] == '#'
        {
            return None;
        }

        let output = self.position;
        self.position = Coord(
            self.position.0 + DIRECTION[self.direction_index].0,
            self.position.1 + DIRECTION[self.direction_index].1,
        );
        return Some(output);
    }
}

fn process_input(input: &str) -> Grid {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn find_guard(grid: &Grid) -> Coord {
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '^' {
                return Coord(x.try_into().unwrap(), y.try_into().unwrap());
            }
        }
    }
    panic!("no guard found");
}

fn solve_simple(input: &str) -> i32 {
    let grid = process_input(input);
    let mut cur_pos = find_guard(&grid);
    let mut direction_index = 0;
    let mut traversed_spaces = grid.clone();

    // let mut turn_count = 0;

    loop {
        // println!("walk cycle: {:?}", turn_count);
        // println!("direction: {:?}", direction_index);
        for coord in (Griderator {
            grid: &grid,
            position: cur_pos,
            direction_index,
        }) {
            let Some((column, row)) = try_usize(coord) else {
                panic!("invalid coord");
            };
            cur_pos = coord;
            traversed_spaces[row][column] = 'X';
        }
        // turn_count += 1;
        // check if still on map before rotating.
        if !on_map(cur_pos, &grid, direction_index) {
            break;
        }
        direction_index = (direction_index + 1) % 4;
    }
    traversed_spaces.iter().flatten().fold(0, |sum, c| match c {
        'X' => sum + 1,
        _ => sum,
    })
}

// len 4 array that records what direction it was passed in, based on DIRECTION index.
type DirectionalSpace = [bool; 4];
type LoopGrid = Vec<Vec<DirectionalSpace>>;

fn detect_loop(grid: &Grid) -> bool {
    let mut cur_pos = find_guard(grid);
    let mut direction_index = 0;
    let mut traversed_spaces: LoopGrid = vec![vec![[false; 4]; grid.len()]; grid.len()];

    // let mut turn_count = 0;

    loop {
        // println!("walk cycle: {:?}", turn_count);
        // println!("direction: {:?}", direction_index);
        for coord in (Griderator {
            grid: &grid,
            position: cur_pos,
            direction_index,
        }) {
            let Some((column, row)) = try_usize(coord) else {
                panic!("invalid coord");
            };
            // if the current space has been visited with the current direction index, a loop has occurred.
            if traversed_spaces[row][column][direction_index] == true {
                return true;
            }
            traversed_spaces[row][column][direction_index] = true;
            cur_pos = coord;
        }
        // turn_count += 1;
        // check if still on map before rotating.
        if !on_map(cur_pos, &grid, direction_index) {
            break;
        }
        direction_index = (direction_index + 1) % 4;
    }
    false
}

fn solve_complex(input: &str) -> i32 {
    // BRAINSTORM encode in the traversed spaces vector a walking direction. If passing a space that was previously where the direction is to the right of the current walk direction, place a O (for the obstacle) in front of the current position, and continue traversing
    // direction can be encoded as just the direction_index, and "rightness" can be checked by if the space passed has a direction of (cur_dir + 1) % 4.
    //
    // Most brute force and slow solution: test placing an obstacle in front of every space visited and see if that forms a cycle.
    // In this solution, a cycle is detected if a space is entered with the same direction it was entered previously, and you can short circuit. Can also skip if the potential obstacle location already has an obstacle there.
    // when you are testing for a cycle, start from the initial guard position for simplicity.
    //
    // Some more complicated cases:
    // 1. It's possible to traverse the same spaces in the opposite direction without looping, so one direction value is insufficient. This may also cause some complications.
    // I think it is sufficient for now that it is INVALID to place an obstacle on any space that was already traversed. Because placing that where it would have obstructed previous movement makes the current position in the path invalid.
    // A possible alternative to this is to cast a line out from a potential O location
    // 2. if there is an unobstructed path to a rightward previously traversed line of spaces, that can also make a loop. Maybe instead of all spaces, instead cast a line to the right after every step, and if encountering an obstacle already
    // encountered with the current direction, then place an obstacle in front. This would make the algorithm be checking obstacles as nodes, instead of spaces as paths.
    // IDEA combine naive solution with complicated case 2.
    // step 1: do the regular path, recording which direction(s) a space is traversed from. create a tentative obstacle anywhere that a line cast to the right finds another line
    let grid = process_input(input);
    let mut cur_pos = find_guard(&grid);
    let mut direction_index = 0;
    let mut looping_obstacles: HashSet<Coord> = HashSet::new();
    loop {
        for coord in (Griderator {
            grid: &grid,
            position: cur_pos,
            direction_index,
        }) {
            let Some((column, row)) = try_usize(coord) else {
                panic!("invalid coord");
            };
            // simulate a run from cur_pos as if there was an obstacle at coord. If it forms a loop, add to the looping_obstacles.
            // Skip anytime the current next and current position are equal. The iterator logic for Griderator means that the last value of one griderator will be the first value of the next griderator.
            if !looping_obstacles.contains(&coord) && coord != cur_pos {
                // println!("testing loop at {:?}", coord);
                let mut modified_grid = grid.clone();
                // add simulated obstacle
                modified_grid[row][column] = '#';
                if detect_loop(&modified_grid) {
                    // println!("loop detected at: {:?}", coord);
                    looping_obstacles.insert(coord);
                }
            }
            cur_pos = coord;
        }
        if !on_map(cur_pos, &grid, direction_index) {
            break;
        }
        direction_index = (direction_index + 1) % 4;
    }
    looping_obstacles.len().try_into().unwrap()
}

pub fn main() {
    println!("{:?}", solve_complex(INPUT_1));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Puzzle tests
    #[test]
    fn solve_simple_input_1() {
        assert_eq!(solve_simple(INPUT_1), 41);
    }

    #[test]
    fn solve_simple_input_2() {
        assert_eq!(solve_simple(INPUT_2), 5461);
    }

    #[test]
    fn solve_complex_input_1() {
        assert_eq!(solve_complex(INPUT_1), 6);
    }

    #[test]
    fn solve_complex_input_2() {
        assert_eq!(solve_complex(INPUT_2), 1836);
    }

    // function tests

    #[test]
    fn find_guard_simple() {
        let grid = r#"
        ...
        ^.#
        ...
        "#;
        let grid = process_input(grid);
        assert_eq!(find_guard(&grid), Coord(0, 1));
    }

    #[test]
    fn griderator_simple() {
        let grid = r#"
        ...
        ^.#
        ...
        "#;
        let grid = process_input(grid);
        let griderator = Griderator {
            grid: &grid,
            position: Coord(0, 1),
            direction_index: 1,
        };
        let grid_vec: Vec<_> = griderator.collect();
        assert_eq!(grid_vec, [Coord(0, 1), Coord(1, 1)].to_vec());
    }

    #[test]
    fn on_map_true() {
        let grid = r#"
        ...
        ..#
        ...
        "#;
        let grid = process_input(grid);
        assert_eq!(on_map(Coord(0, 0), &grid, 1), true);
    }

    #[test]
    fn on_map_false_too_small() {
        let grid = r#"
        ...
        ..#
        ...
        "#;
        let grid = process_input(grid);
        assert_eq!(on_map(Coord(0, 0), &grid, 0), false);
    }

    #[test]
    fn on_map_false_too_large() {
        let grid = r#"
        ...
        ..#
        ...
        "#;
        let grid = process_input(grid);
        assert_eq!(on_map(Coord(2, 0), &grid, 1), false);
    }
}

const INPUT_1: &str = r#"
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"#;

const INPUT_2: &str = r#"
....#.................#......................#..........................#..................#....##..#...........#.................
...................................#...............................#......#..#...............................#....................
..........................#................#......##.....#.....................................#...............#..#...............
.......................................................................................................#..........................
...................#....#.........................#..............#.....#......................................................#...
.....#.........................................................................................................#..................
.........................................................#....................#..#............#................#..................
..............#...............#..................................................................#......#.........................
.........#.....#.......#......#.......................#.............#..#........#.......#............#.......#....#..#.......#....
........................#....#...............................................#...#.#.........................................#....
.........................#........................................................................#.....................#.........
.#..........#...#...............#..................#...#......#.................................#....................#.......#....
............................................................#......#.........................................................#....
.......................#...#..................#.#.............#......................#.............#..............................
............................................................................................................#.........#...........
....#..................###........#.............................#.#....................#..........#.....................#.........
................................................................................................#.......................#.#.......
..........................................................................#........#...#.................#.......#................
..........................#.#..#.#...............................................#..........#..............................#......
..............#........#.#...............#............................#..........#....................................#........#.#
#..................................................................................................#...........#................#.
...............................................#........................................#.................................#.......
.....#..#......#.......#..............#........#........#...........................................................#.............
.......#..........#.......#.#...........#................#.................................................#......................
....#.....#...............#....#.......#...#.............................................................#...................#....
............................................#....#.......#................................#......#......................#.....#...
.....................#...................................#....#...........................................................#.......
..#..............................#...........#...........................#.................................#......................
........#.............................##...................#.....................#................................................
................#.#.#......#...............................................#...###.........#....#.................................
.......#.............#.............#...........#..#...............................#..................................#.....#.....#
.....#...#.......#.........................................#..............................#........................#.............#
...............................................#.....#..............................................................#.............
.........#.......................................#.......................#...........#............................................
.......................#.....#.....................................................#....................................#.........
.............................#......................#.....................#..............#..#.......................#.............
......#......................................#.......#.....................#.....#...........#..........#.....................#...
..................................................#....................#.......#....#.......#...#.................................
......................................#......#........#.....#........................................#.............#..............
#......................#.#..............#.....#.........#...............................#...............................#.........
#......#...............#.........................................................................................#..#.............
.#..............................................#.................#.......#.......................................................
...#.......#.....#.............#...................................................................................#..............
..........................#........#........#.....................................#........................#..............#..#....
.......#..........#.......#...................................................#.##.....#.#.........#....#.................#.......
..##...................#.......................................#..............#.#........#........................................
#.......#..............#...........#.....#....#....#............#........#..................................#.#.........#.........
..................#.......................................................................................................#.......
............................#.........#....................#...............................#..........#...........................
.............#.....................................................................#.....#.................#......................
...#...#............#..........................#..........................................#.......................................
..#.............#.#........................#...#.......................#.........#....................##....#.....................
...........................................................................................................#.........#...#........
.............................#.#..........................................................................................#.......
....................#........................#.#......#...............#......#.............................#......................
....#...#.......#..................#..........................................................#........................#..........
.#...........#..............................................#...........................................#.........................
..............................#...........#...........................................#.........#...........................#....#
............#.......#......#....#.......................................................................#.........................
.........................................#...#..................#..................#.....................................#........
................#..................#........#.........................#...................#......................................#
............................................................#.....#........#......................................................
..............................#............#.........................................#.............#.......#..#...................
...#............#........................................................................#..................................#....#
..............................#.......#.............#.................................................#...........................
.....#.................................................#..........#...............................#..##..........................#
.............#.........................................................#.............................#..#..............#........#.
....................#...............#................................................................................#....#......#
........#....#..#...........................................................................#................#..............#.....
....#......................#...............#..........#......#.........#..^....#..........#..................#.......#............
...........................#.#...................................................#...............................#................
#..#.........................................................#.....#........................#.....................................
.#.#....#.......................................#......#..........#................#..#...........................................
...#.....#..........#...................#.......#............................#....................................................
.............#......................................................#............#.....................#.....................#....
...................#.............................................................................................#................
...................#.....#...................................................................................................#....
................#............#......................#.......#..................#..#.....................#..............#........#.
#....#...................................................................#.......##....#..........#...............................
.........................................#............#......#.......................#........#..............#.#..................
.##........#.....#...........................................................................................#..............#.....
#...........................................................#.......#....#......#..#...................................#..#.#.....
.............#........................................#...............................#...........................................
......................#..................................................................#..................................#.....
...#.#........................................#.................................................................................#.
.............#.......................#............................#.#................................#.......#..#...........#.....
...#..#...............................................................................#................#..........................
.......#.............................#............#..#........#.....#.........................#..................#................
......#...............................................................................##.................................#....#...
........#..................#.......................................................................#......#..........#............
...........#.......#........#..........................................#........#..............#.................#.........#......
...................#..............................#...............................#.....................#.....#...................
.............#..#..................#.#..#.....#..#..........................................................................#.....
..............................#...................................................#.......................#..............#........
.......#......................#.............................#...........................#.....#...#.................#........#....
.........#.......#....##..........................................................................................................
........................................................................#......................#..................................
.......................#..........##.#................................................#..#.....#......#......#.................#..
......................................#......................................................................#....................
.....................................#....#...............................................#...#.....#.......#...#...........#.....
...................#..#................................................................................#..........................
..............#.................#..............................#..............#...........#.#..................#...#.............#
................#..#......#.........................#.....#.....#..............#..#.........................................#.....
.....#.........................#........................................#.....#.......#...........#...................#...........
......#....................##......................................................................................#..............
............................................................................###...............................#...................
#..................................#......#..............................................................................#.......#
.....#...................#.....#............................................................#.....................#...#...........
.......#......#........................#.....................#.#..##............#......#.................#.....................#..
.....#..............#.................#........#..................#..#..........................##.....#..#...................#...
........#................#................#...........#.........................................#....................#............
.........#.........#....#................#........#........#...............#....#.................................................
...........#...#............#..................#.............................#.#..#....................#.#........................
.#............#..#..#................#.#................................................................##.............#..........
...........#.......................#.........................................................#....................................
..#..........................#...........#......#................................#.#..............................................
............................................................#..................#...........#.....##.................#.............
....#.........#............................##..........................................#.....#....#..........#....................
..................#..............#........#........#....................#..............#....................#.....................
....#...................................#.....................................................#.............#...................#.
.........#...........#...#.............................#.............##..#................................................#.#.....
............................##..............#................#..#........................................#....................#...
.............................#.#......................................................##.....#..................................#.
.............................................................................#..................#...#.................#...........
........................#...........#..................#......#...................................................................
.....#.......................#......#................#............#..................#.................................#..........
#..........#...................................................................#...................#...#...............#..........
....................#............................................#...............#........................##....#..#..............
.#.....#............................#...............#.#.#..........................#.........#..............#.....................
.............................#....................................#..#......#........................#............................
"#;

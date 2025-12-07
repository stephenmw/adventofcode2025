use ahash::AHashMap;

use crate::solutions::prelude::*;

use crate::grid::{Direction, Grid, Point};

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    let start = grid
        .iter_items()
        .find(|(_, c)| **c == Cell::Start)
        .unwrap()
        .0;

    let mut beams = vec![start];
    let mut total_splits = 0;
    while !beams.is_empty() {
        let (new_beams, splits) = step(&grid, &beams);
        total_splits += splits;
        beams = new_beams;
    }

    Ok(total_splits.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    let start = grid
        .iter_items()
        .find(|(_, c)| **c == Cell::Start)
        .unwrap()
        .0;

    let mut beams = AHashMap::new();
    beams.insert(start, 1);
    loop {
        let new_beams = step2(&grid, &beams);
        if new_beams.is_empty() {
            return Ok(beams.values().copied().sum::<usize>().to_string());
        }
        beams = new_beams;
    }
}

fn step(grid: &Grid<Cell>, beams: &[Point]) -> (Vec<Point>, usize) {
    let mut new_beams = Vec::new();
    let mut splits = 0;

    for beam in beams {
        let Some(next_pos) = beam.next(Direction::Up) else {
            continue;
        };

        match grid.get(next_pos) {
            Some(Cell::Empty) | Some(Cell::Start) => {
                new_beams.push(next_pos);
            }
            Some(Cell::Splitter) => {
                splits += 1;
                if let Some(left) = next_pos.next(Direction::Left) {
                    new_beams.push(left);
                }
                if let Some(right) = next_pos.next(Direction::Right) {
                    new_beams.push(right);
                }
            }
            None => continue,
        }
    }

    new_beams.sort_unstable();
    new_beams.dedup();

    (new_beams, splits)
}

fn step2(grid: &Grid<Cell>, beams: &AHashMap<Point, usize>) -> AHashMap<Point, usize> {
    let mut new_beams = AHashMap::default();

    for (beam, count) in beams {
        let Some(next_pos) = beam.next(Direction::Up) else {
            continue;
        };

        match grid.get(next_pos) {
            Some(Cell::Empty) | Some(Cell::Start) => {
                *new_beams.entry(next_pos).or_default() += *count;
            }
            Some(Cell::Splitter) => {
                if let Some(left) = next_pos.next(Direction::Left) {
                    *new_beams.entry(left).or_default() += *count;
                }
                if let Some(right) = next_pos.next(Direction::Right) {
                    *new_beams.entry(right).or_default() += *count;
                }
            }
            None => continue,
        }
    }

    new_beams
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Start,
    Empty,
    Splitter,
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Grid<Cell>> {
        let cell = alt((
            value(Cell::Start, char('S')),
            value(Cell::Splitter, char('^')),
            value(Cell::Empty, char('.')),
        ));
        let row = many1(cell);
        let grid = many1(ws_line(row)).map(|rows| Grid::new(rows));
        ws_all_consuming(grid).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = ".......S.......
    ...............
    .......^.......
    ...............
    ......^.^......
    ...............
    .....^.^.^.....
    ...............
    ....^.^...^....
    ...............
    ...^.^...^.^...
    ...............
    ..^...^.....^..
    ...............
    .^.^.^.^.^...^.
    ...............";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "21")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "40")
    }
}

use ahash::AHashMap;

use crate::solutions::prelude::*;

use crate::util::grid::{Direction, Grid, Point};

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (splitters_hit, _) = solve(input)?;
    Ok(splitters_hit.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (_, total_worlds) = solve(input)?;
    Ok(total_worlds.to_string())
}

fn solve(input: &str) -> Result<(usize, usize), anyhow::Error> {
    let grid = parse!(input);
    let start = grid
        .iter_items()
        .find(|(_, c)| **c == Cell::Start)
        .ok_or_else(|| anyhow::anyhow!("Start cell not found"))?
        .0;

    let mut beams = AHashMap::new();
    let mut total_splitters_hit = 0;
    beams.insert(start, 1);
    loop {
        let (new_beams, spliters_hit) = step(&grid, &beams);
        total_splitters_hit += spliters_hit;
        if new_beams.is_empty() {
            let total_worlds = beams.values().copied().sum::<usize>();
            return Ok((total_splitters_hit, total_worlds));
        }
        beams = new_beams;
    }
}

fn step(grid: &Grid<Cell>, beams: &AHashMap<Point, usize>) -> (AHashMap<Point, usize>, usize) {
    let mut new_beams = AHashMap::default();
    let mut spliters_hit = 0;

    for (beam, count) in beams {
        let Some(next_pos) = beam.next(Direction::Up) else {
            continue;
        };

        match grid.get(next_pos) {
            Some(Cell::Empty) | Some(Cell::Start) => {
                *new_beams.entry(next_pos).or_default() += *count;
            }
            Some(Cell::Splitter) => {
                spliters_hit += 1;
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

    (new_beams, spliters_hit)
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
        let grid = many1(ws_line(row)).map(Grid::new);
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

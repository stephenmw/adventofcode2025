use crate::solutions::prelude::*;

use crate::util::grid::{Grid, Point};

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let grid = parse!(input);
    let movable = movable_cells(&grid);

    Ok(movable.len().to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let mut grid = parse!(input);
    let mut removed = 0;

    loop {
        let movable = movable_cells(&grid);
        if movable.is_empty() {
            break;
        }

        removed += movable.len();

        for loc in movable {
            *grid.get_mut(loc).unwrap() = Cell::Empty;
        }
    }

    Ok(removed.to_string())
}

fn movable_cells(grid: &Grid<Cell>) -> Vec<Point> {
    let mut movable = Vec::new();
    for (loc, v) in grid.iter_items() {
        if *v == Cell::Occupied {
            let num_occupied_neighbors = loc
                .iter_adjacent8()
                .filter_map(|x| grid.get(x))
                .filter(|&c| *c == Cell::Occupied)
                .count();
            if num_occupied_neighbors < 4 {
                movable.push(loc);
            }
        }
    }

    movable
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Occupied,
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Grid<Cell>> {
        let cell = alt((
            char('.').map(|_| Cell::Empty),
            char('@').map(|_| Cell::Occupied),
        ));
        let row = ws_line(many1(cell));
        let grid = many1(row).map(Grid::new);
        ws_all_consuming(grid).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "..@@.@@@@.
    @@@.@.@.@@
    @@@@@.@.@@
    @.@@@@..@.
    @@.@@@@.@@
    .@@@@@@@.@
    .@.@.@.@@@
    @.@@@.@@@@
    .@@@@@@@@.
    @.@.@@@.@.";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "13")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "43")
    }
}

use std::{cmp::Ordering, fmt::Display};

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);

    let mut cur = 50;
    let mut cnt = 0;

    for r in data {
        cur = rotate(cur, r);
        if cur == 0 {
            cnt += 1;
        }
    }

    Ok(cnt.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);

    let mut cur = 50;
    let mut cnt = 0;

    for r in data {
        let n = match r.dir {
            Direction::Left => cur - r.dist,
            Direction::Right => cur + r.dist,
        };

        cnt += match n.cmp(&0) {
            Ordering::Less => n.abs() / 100 + if cur == 0 { 0 } else { 1 },
            Ordering::Equal => 1,
            Ordering::Greater => n / 100,
        };

        cur = n.rem_euclid(100);
    }

    Ok(cnt.to_string())
}

fn rotate(cur: i32, rotation: Rotation) -> i32 {
    match rotation.dir {
        Direction::Left => cur - rotation.dist,
        Direction::Right => cur + rotation.dist,
    }
    .rem_euclid(100)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rotation {
    dir: Direction,
    dist: i32,
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            match self.dir {
                Direction::Left => "L",
                Direction::Right => "R",
            },
            self.dist
        )
    }
}

mod parser {
    use nom::{combinator::all_consuming, sequence::pair};

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Rotation>> {
        let direction = alt((
            value(Direction::Left, char('L')),
            value(Direction::Right, char('R')),
        ));
        let rotation = pair(direction, uint()).map(|(d, n)| Rotation { dir: d, dist: n });
        let lines = many1(ws_line(rotation));
        all_consuming(lines).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "3")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "6")
    }
}

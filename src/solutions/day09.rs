use crate::solutions::prelude::*;

use crate::grid::Point;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let points = parse!(input);

    let pairs = (0..points.len())
        .flat_map(|i| (i + 1..points.len()).map(move |j| (i, j)))
        .map(|(i, j)| (points[i], points[j]));

    let max_area = pairs
        .map(|(p1, p2)| (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1))
        .max()
        .ok_or_else(|| anyhow::anyhow!("no points provided"))?;

    Ok(max_area.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let points = parse!(input);
    let candidates = candidates_by_area(&points);
    let row_ranges = ranges_per_row(&points);

    for (a, b, area) in candidates {
        if check_rectangle(a, b, &row_ranges) {
            return Ok(area.to_string());
        }
    }

    bail!("no solution")
}

fn check_rectangle(a: Point, b: Point, row_ranges: &[Option<InclusiveRange>]) -> bool {
    let y_range = InclusiveRange::new(a.y, b.y);
    let x_range = InclusiveRange::new(a.x, b.x);

    y_range
        .iter()
        .all(|y| row_ranges[y].map(|r| r.contains(&x_range)).unwrap_or(false))
}

fn candidates_by_area(points: &[Point]) -> Vec<(Point, Point, usize)> {
    let mut candidates = Vec::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = points[i];
            let p2 = points[j];
            let area = (p1.x.abs_diff(p2.x) + 1) * (p1.y.abs_diff(p2.y) + 1);
            candidates.push((p1, p2, area));
        }
    }

    candidates.sort_by_key(|&(_, _, area)| std::cmp::Reverse(area));
    candidates
}

fn ranges_per_row(points: &[Point]) -> Vec<Option<InclusiveRange>> {
    let max_y = points.iter().map(|p| p.y).max().unwrap_or(0);

    let last_pair = [*points.first().unwrap(), *points.last().unwrap()];
    let pairs = points.windows(2).chain(std::iter::once(last_pair.as_ref()));
    pairs
        .flat_map(|w| {
            let p1 = &w[0];
            let p2 = &w[1];

            InclusiveRange::new(p1.y, p2.y)
                .iter()
                .map(|y| (y, InclusiveRange::new(p1.x, p2.x)))
        })
        .fold(vec![None; max_y + 1], |mut acc, (y, x_range)| {
            acc[y] = Some(acc[y].map(|r| r.merge(&x_range)).unwrap_or(x_range));
            acc
        })
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct InclusiveRange {
    start: usize,
    end: usize,
}

impl InclusiveRange {
    fn new(a: usize, b: usize) -> Self {
        Self {
            start: a.min(b),
            end: a.max(b),
        }
    }

    fn merge(&self, other: &Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    fn contains(&self, other: &Self) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn iter(&self) -> impl Iterator<Item = usize> + use<> {
        self.start..=self.end
    }
}

impl std::fmt::Debug for InclusiveRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.start, self.end)
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Point>> {
        let point = separated_pair(uint(), char(','), uint()).map(|(x, y)| Point { x, y });
        let points = many1(ws_line(point));
        ws_all_consuming(points).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "7,1
    11,1
    11,7
    9,7
    9,5
    2,5
    2,3
    7,3";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "50")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "24")
    }
}

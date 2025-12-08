use ahash::AHashMap;

use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    problem1_(input, 1000)
}

fn problem1_(input: &str, n: usize) -> Result<String, anyhow::Error> {
    let points = parse!(input);

    let distances = pairs_by_distance(&points);

    let pairs_to_consider = distances[..n.min(distances.len())]
        .iter()
        .map(|(pair, _)| *pair);

    let mut groups = Groups::default();
    for pair in pairs_to_consider {
        groups.connect(&pair.0, &pair.1);
    }

    let sorted_freq = {
        let mut freq = groups.iter().map(|group| group.len()).collect::<Vec<_>>();
        freq.sort_unstable();
        freq
    };

    let ans = sorted_freq[sorted_freq.len() - 3..]
        .iter()
        .product::<usize>();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let points = parse!(input);

    let distances = pairs_by_distance(&points);

    let pairs_to_consider = distances.iter().map(|(pair, _)| *pair);

    let mut groups = Groups::default();
    (0..points.len()).for_each(|i| {
        groups.add_group(i);
    });
    for pair in pairs_to_consider {
        groups.connect(&pair.0, &pair.1);
        if groups.len == 1 {
            let ans = points[pair.0].x * points[pair.1].x;
            return Ok(ans.to_string());
        }
    }

    unreachable!()
}

fn pairs_by_distance(points: &[Point]) -> Vec<((usize, usize), u64)> {
    let pairs = (0..points.len()).flat_map(|i| (i + 1..points.len()).map(move |j| (i, j)));

    let mut distances: Vec<_> = pairs
        .map(|(i, j)| ((i, j), points[i].distance(&points[j])))
        .collect();
    distances.sort_unstable_by_key(|(_, distance)| *distance);

    distances
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
    z: u64,
}

impl Point {
    fn distance(&self, other: &Point) -> u64 {
        (self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2))
        .isqrt()
    }
}

#[derive(Clone, Debug, Default)]
struct Groups<T> {
    groups: Vec<Vec<T>>,
    lookup_table: AHashMap<T, usize>,
    len: usize,
}

impl<T: Eq + std::hash::Hash + Clone> Groups<T> {
    fn add_group(&mut self, first_item: T) -> usize {
        let new_id = self.groups.len();
        self.lookup_table.insert(first_item.clone(), new_id);
        self.groups.push(vec![first_item]);
        self.len += 1;
        new_id
    }

    fn connect(&mut self, a: &T, b: &T) {
        let group_a = self.lookup_table.get(a).copied();
        let group_b = self.lookup_table.get(b).copied();

        match (group_a, group_b) {
            (Some(ga), Some(gb)) => {
                self.merge_groups(ga, gb);
            }
            (Some(ga), None) => {
                self.assign_to_group(b.clone(), ga);
            }
            (None, Some(gb)) => {
                self.assign_to_group(a.clone(), gb);
            }
            (None, None) => {
                let group = self.add_group(a.clone());
                self.assign_to_group(b.clone(), group);
            }
        }
    }

    fn iter(&self) -> impl Iterator<Item = &Vec<T>> {
        self.groups.iter().filter(|x| !x.is_empty())
    }

    fn merge_groups(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }

        // Ensure a is the larger group
        let (a, b) = if self.groups[a].len() > self.groups[b].len() {
            (a, b)
        } else {
            (b, a)
        };

        let b_values = std::mem::take(&mut self.groups[b]);

        for v in &b_values {
            *self.lookup_table.get_mut(v).unwrap() = a;
        }
        self.groups[a].extend(b_values);
        self.len -= 1;
    }

    fn assign_to_group(&mut self, item: T, group_id: usize) {
        self.lookup_table.insert(item.clone(), group_id);
        self.groups[group_id].push(item);
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Point>> {
        let point =
            (uint(), char(','), uint(), char(','), uint()).map(|(x, _, y, _, z)| Point { x, y, z });
        let points = many1(ws_line(point));
        ws_all_consuming(points).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "162,817,812
    57,618,57
    906,360,560
    592,479,940
    352,342,300
    466,668,158
    542,29,236
    431,825,988
    739,650,466
    52,470,668
    216,146,977
    819,987,18
    117,168,530
    805,96,715
    346,949,466
    970,615,88
    941,993,340
    862,61,35
    984,92,344
    425,690,689";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1_(EXAMPLE_INPUT, 10).unwrap(), "40")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "25272")
    }
}

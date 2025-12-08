use crate::{solutions::prelude::*, utils::freq_table};

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    problem1_(input, 1000)
}

fn problem1_(input: &str, n: usize) -> Result<String, anyhow::Error> {
    let points = parse!(input);

    let pairs = (0..points.len()).flat_map(|i| (i + 1..points.len()).map(move |j| (i, j)));

    let mut distances = pairs
        .map(|(i, j)| ((i, j), points[i].distance(&points[j])))
        .collect::<Vec<_>>();
    distances.sort_unstable_by_key(|(_, distance)| *distance);

    let pairs_to_consider = distances[..n.min(distances.len())]
        .iter()
        .map(|(pair, _)| *pair);

    let mut assigned_circuit: Vec<_> = (0..points.len()).collect();
    for pair in pairs_to_consider {
        if assigned_circuit[pair.0] != assigned_circuit[pair.1] {
            let new_id = assigned_circuit[pair.0];
            let old_id = assigned_circuit[pair.1];
            assigned_circuit.iter_mut().for_each(|id| {
                if *id == old_id {
                    *id = new_id;
                }
            });
        }
    }

    let sorted_freq = {
        let freq = freq_table(assigned_circuit);
        let mut freq_vec: Vec<_> = freq.into_iter().collect();
        freq_vec.sort_unstable_by_key(|&(_, count)| std::cmp::Reverse(count));
        freq_vec
    };

    let ans = sorted_freq[..3]
        .iter()
        .map(|&(_, count)| count as u64)
        .product::<u64>();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let points = parse!(input);

    let pairs = (0..points.len()).flat_map(|i| (i + 1..points.len()).map(move |j| (i, j)));

    let mut distances = pairs
        .map(|(i, j)| ((i, j), points[i].distance(&points[j])))
        .collect::<Vec<_>>();
    distances.sort_unstable_by_key(|(_, distance)| *distance);

    let pairs_to_consider = distances.iter().map(|(pair, _)| *pair);

    let mut assigned_circuit: Vec<_> = (0..points.len()).collect();
    for pair in pairs_to_consider {
        if assigned_circuit[pair.0] != assigned_circuit[pair.1] {
            let new_id = assigned_circuit[pair.0];
            let old_id = assigned_circuit[pair.1];
            assigned_circuit.iter_mut().for_each(|id| {
                if *id == old_id {
                    *id = new_id;
                }
            });
        }

        if assigned_circuit.iter().all(|&id| id == assigned_circuit[0]) {
            let ans = points[pair.0].x * points[pair.1].x;
            return Ok(ans.to_string());
        }
    }

    unreachable!()
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

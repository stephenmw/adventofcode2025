use crate::solutions::prelude::*;

use ahash::HashMap;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let problems = parse!(input);

    let ans: u64 = problems.iter().map(|p| p.solve()).sum();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let problems = parse!(input);

    let ans: u64 = problems.iter().map(|p| p.solve2()).sum();
    Ok(ans.to_string())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Mul,
    Add,
}

impl Op {
    fn apply(&self, xs: impl Iterator<Item = u64>) -> u64 {
        match self {
            Op::Mul => xs.product(),
            Op::Add => xs.sum(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Problem {
    xs: Vec<Vec<(usize, u8)>>,
    op: Op,
}

impl Problem {
    fn solve(&self) -> u64 {
        let nums = self.xs.iter().map(|v| {
            v.iter()
                .map(|&(_, n)| n as u64)
                .fold(0u64, |acc, x| acc * 10 + x)
        });

        self.op.apply(nums)
    }

    fn solve2(&self) -> u64 {
        let nums =
            self.xs
                .iter()
                .flat_map(|xs| xs.iter())
                .fold(HashMap::default(), |mut acc, &(i, n)| {
                    let entry = acc.entry(i).or_insert(0u64);
                    *entry = *entry * 10 + n as u64;
                    acc
                });

        self.op.apply(nums.values().copied())
    }
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Problem>> {
        nom::combinator::rest
            .map(|rest: &str| parse_input(rest))
            .parse_complete(input)
    }

    fn parse_input(input: &str) -> Vec<Problem> {
        let lines: Vec<_> = input.lines().map(parse_line).collect();
        let Some((op_line, num_lines)) = lines.split_last() else {
            panic!("no op line");
        };

        let mut problems: Vec<Problem> = op_line
            .iter()
            .map(|(_, x)| match x {
                b'*' => Op::Mul,
                b'+' => Op::Add,
                _ => panic!("bad op"),
            })
            .map(|op| Problem {
                xs: vec![vec![]; num_lines.len()],
                op,
            })
            .collect();

        let col_starts: Vec<_> = op_line.iter().map(|x| x.0).collect();

        for (i, line) in num_lines.iter().enumerate() {
            for (j, n) in line.iter() {
                let col = col_starts.partition_point(|x| x <= j) - 1;
                problems[col].xs[i].push((*j, n - b'0'));
            }
        }

        problems
    }

    fn parse_line(line: &str) -> Vec<(usize, u8)> {
        line.bytes()
            .enumerate()
            .filter(|(_, c)| !c.is_ascii_whitespace())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "4277556")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "3263827")
    }
}

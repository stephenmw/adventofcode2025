use ahash::HashSet;
use rand::rand_core::le;

use crate::solutions::prelude::*;

use crate::range::Range;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (ranges, ingredients) = parse!(input);
    let ans: usize = ingredients
        .iter()
        .filter(|&ingredient| ranges.iter().any(|r| r.contains(*ingredient)))
        .count();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (mut ranges, _) = parse!(input);
    ranges.sort_by_key(|x| x.start);

    let merged_ranges = ranges.iter().fold(Vec::new(), |mut acc, r| {
        let Some(last) = acc.last_mut() else {
            acc.push(*r);
            return acc;
        };

        match last.merge(r) {
            Some(merged) => *last = merged,
            None => acc.push(*r),
        };

        acc
    });

    let ans: u64 = merged_ranges.iter().map(|r| r.length()).sum();
    Ok(ans.to_string())
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, (Vec<Range>, Vec<u64>)> {
        let range =
            separated_pair(uint(), char('-'), uint::<u64>()).map(|(a, b)| Range::new(a, b + 1));
        let ranges = many1(ws_line(range));
        let ingredients = many1(ws_line(uint()));
        let parser = separated_pair(ranges, multispace1, ingredients);
        ws_all_consuming(parser).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3-5
    10-14
    16-20
    12-18

    1
    5
    8
    11
    17
    32";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "3")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "14")
    }
}

use crate::solutions::prelude::*;

use crate::range::{Range, RangeSet};

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let (ranges, ingredients) = parse!(input);
    let set = RangeSet::from(ranges);
    let ans: usize = ingredients
        .iter()
        .filter(|&ingredient| set.contains(*ingredient))
        .count();
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let (ranges, _) = parse!(input);
    let set = RangeSet::from(ranges);
    Ok(set.len().to_string())
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

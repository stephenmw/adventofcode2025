use crate::solutions::prelude::*;

use crate::range::Range;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    solve(input, is_valid)
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    solve(input, is_valid2)
}

fn solve(input: &str, valid_fn: fn(u64) -> bool) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let ans: u64 = data
        .iter()
        .flat_map(|r| r.iter().filter(|&n| !valid_fn(n)))
        .sum();
    Ok(ans.to_string())
}

fn is_valid(n: u64) -> bool {
    let s = n.to_string();
    if s.len() % 2 == 1 {
        return true;
    }

    let mid = s.len() / 2;
    let (a, b) = s.split_at(mid);
    a != b
}

fn is_valid2(n: u64) -> bool {
    let s = n.to_string().into_bytes();
    for pattern_size in 1..=s.len() / 2 {
        if s.len().is_multiple_of(pattern_size) {
            let (pattern, rest) = s.split_at(pattern_size);
            if rest.chunks(pattern_size).all(|x| x == pattern) {
                return false;
            }
        }
    }

    true
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Range>> {
        let range = separated_pair(uint(), char('-'), uint::<u64>())
            .map(|(start, end)| Range::new(start, end + 1));
        let ranges = separated_list1(delimited(multispace0, char(','), multispace0), range);
        ws_all_consuming(ranges).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    824824821-824824827,2121212118-2121212124";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "1227775554")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "4174379265")
    }
}

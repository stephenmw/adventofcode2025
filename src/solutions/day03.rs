use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let batteries = parse!(input);

    let ans: u64 = batteries
        .iter()
        .map(|battery| largest_output_n_digits(battery, 2))
        .sum();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let batteries = parse!(input);

    let ans: u64 = batteries
        .iter()
        .map(|battery| largest_output_n_digits(battery, 12))
        .sum();

    Ok(ans.to_string())
}

fn largest_output_n_digits(digits: &[u8], n: usize) -> u64 {
    if n == 0 {
        return 0;
    }

    let (pos, d) = first_largest_digit(&digits[..digits.len() - n + 1]).unwrap();
    let next_digits = largest_output_n_digits(&digits[pos + 1..], n - 1);
    d as u64 * 10u64.pow(n as u32 - 1) + next_digits
}

fn first_largest_digit(digits: &[u8]) -> Option<(usize, u8)> {
    let mut pos = 0;
    let mut max_digit = digits.first().copied()?;

    for (i, d) in digits.iter().copied().enumerate() {
        if d > max_digit {
            pos = i;
            max_digit = d;
        }

        if max_digit == 9 {
            break;
        }
    }

    Some((pos, max_digit))
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
        let digit = one_of("0123456789").map(|c| c.to_string().parse().unwrap());
        let line = ws_line(many1(digit));
        let lines = many1(line);
        ws_all_consuming(lines).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "987654321111111
    811111111111119
    234234234234278
    818181911112111";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "357")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "3121910778619")
    }
}

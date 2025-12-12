use crate::solutions::prelude::*;

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let data = parse!(input);
    let ans = data
        .iter()
        .filter(|((l, w), xs)| xs.iter().sum::<usize>() * 7 < l * w)
        .count();

    Ok(ans.to_string())
}

pub fn problem2(_input: &str) -> Result<String, anyhow::Error> {
    Ok("Finish Decorating the North Pole".to_string())
}

mod parser {
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<((usize, usize), Vec<usize>)>> {
        let skipped_input = input.splitn(30, '\n').last().unwrap();

        let num_list = separated_list1(space1, uint());
        let dimensions = separated_pair(uint(), char('x'), uint());
        let tree = separated_pair(dimensions, (char(':'), space0), num_list);
        let trees = many1(ws_line(tree));
        ws_all_consuming(trees).parse_complete(skipped_input)
    }
}

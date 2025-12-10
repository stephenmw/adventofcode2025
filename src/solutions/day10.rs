use crate::solutions::prelude::*;

use ahash::AHashSet;
use good_lp::{
    Expression, Solution, SolverModel, microlp,
    variable::{ProblemVariables, VariableDefinition},
};

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let machines = parse!(input);

    let ans: usize = machines.iter().map(|m| num_buttons_indicators(m)).sum();

    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let machines = parse!(input);

    let ans: usize = machines.iter().map(|m| num_buttons_joltages(m)).sum();

    Ok(ans.to_string())
}

fn num_buttons_indicators(m: &Machine) -> usize {
    let mut next_state: AHashSet<Vec<bool>> = AHashSet::new();
    next_state.insert(vec![false; m.indicators.len()]);
    let mut presses = 0;

    let mut state = AHashSet::new();

    loop {
        std::mem::swap(&mut state, &mut next_state);
        presses += 1;

        for s in state.drain() {
            for b in m.buttons.iter() {
                let ns = apply_button(&s, b);
                if ns == m.indicators {
                    return presses;
                }
                next_state.insert(ns);
            }
        }
    }
}

fn apply_button(indicators: &[bool], button: &[usize]) -> Vec<bool> {
    let mut ret = indicators.to_vec();
    for &i in button {
        ret[i] = !ret[i];
    }

    ret
}

fn num_buttons_joltages(m: &Machine) -> usize {
    let mut pv = ProblemVariables::new();
    let vars = pv.add_vector(VariableDefinition::new().integer().min(0), m.buttons.len());

    let joltage_expressions = m.buttons.iter().zip(&vars).fold(
        vec![Expression::with_capacity(vars.len()); m.joltages.len()],
        |mut acc, (button, var)| {
            for &joltage_id in button {
                acc[joltage_id] += var
            }
            acc
        },
    );

    let constrants = joltage_expressions
        .into_iter()
        .zip(&m.joltages)
        .map(|(exp, &rhs)| exp.eq(rhs as i32));

    let problem_expr: Expression = vars.iter().sum();

    let model = pv
        .minimise(problem_expr)
        .using(microlp)
        .with_all(constrants);

    let solution = model.solve().unwrap();

    vars.iter()
        .map(|v| solution.value(*v).round() as usize)
        .sum()
}

#[derive(Clone, Debug)]
struct Machine {
    indicators: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

mod parser {
    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
        let light = alt((value(false, char('.')), value(true, char('#'))));
        let indicators = delimited(char('['), many1(light), char(']'));

        let num_list = || separated_list1(char(','), uint());
        let button = delimited(char('('), num_list(), char(')'));
        let buttons = separated_list1(space0, button);

        let joltages = delimited(char('{'), num_list(), char('}'));

        let machine = (indicators, space0, buttons, space0, joltages).map(
            |(indicators, _, buttons, _, joltages)| Machine {
                indicators,
                buttons,
                joltages,
            },
        );
        let machines = many1(ws_line(machine));

        ws_all_consuming(machines).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "7")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT).unwrap(), "33")
    }
}

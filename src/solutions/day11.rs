use crate::solutions::prelude::*;

use ahash::{AHashMap, AHashSet};

pub fn problem1(input: &str) -> Result<String, anyhow::Error> {
    let graph = parse!(input);

    fn rec<'a>(
        graph: &'a AHashMap<&str, Vec<&str>>,
        seen: &mut AHashSet<&'a str>,
        node: &'a str,
    ) -> usize {
        if node == "out" {
            return 1;
        }

        if !seen.insert(node) {
            return 0;
        }

        let ret: usize = graph
            .get(node)
            .unwrap()
            .iter()
            .map(|&child| rec(graph, seen, child))
            .sum();

        seen.remove(node);
        ret
    }
    let ans = rec(&graph, &mut AHashSet::new(), "you");
    Ok(ans.to_string())
}

pub fn problem2(input: &str) -> Result<String, anyhow::Error> {
    let graph = parse!(input);

    fn rec<'a>(
        graph: &'a AHashMap<&str, Vec<&str>>,
        seen: &mut AHashSet<&'a str>,
        node: &'a str,
        memo: &mut AHashMap<(&'a str, bool, bool), usize>,
    ) -> usize {
        if node == "out" {
            if seen.contains("dac") && seen.contains("fft") {
                return 1;
            } else {
                return 0;
            }
        }

        if let Some(&ans) = memo.get(&(node, seen.contains("dac"), seen.contains("fft"))) {
            return ans;
        }

        if !seen.insert(node) {
            return 0;
        }

        let ret: usize = graph
            .get(node)
            .unwrap()
            .iter()
            .map(|&child| rec(graph, seen, child, memo))
            .sum();

        seen.remove(node);
        memo.insert((node, seen.contains("dac"), seen.contains("fft")), ret);
        ret
    }
    let ans = rec(&graph, &mut AHashSet::new(), "svr", &mut AHashMap::new());
    Ok(ans.to_string())
}

mod parser {
    use nom::character::complete::alpha1;

    use super::*;
    use crate::parser::prelude::*;

    pub fn parse(input: &str) -> IResult<&str, AHashMap<&str, Vec<&str>>> {
        let node_list = separated_list1(space1, alpha1);
        let nodedef = separated_pair(alpha1, (char(':'), space0), node_list);
        let graph = fold_many1(ws_line(nodedef), AHashMap::new, |mut acc, (n, l)| {
            acc.insert(n, l);
            acc
        });
        ws_all_consuming(graph).parse_complete(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "aaa: you hhh
    you: bbb ccc
    bbb: ddd eee
    ccc: ddd eee fff
    ddd: ggg
    eee: out
    fff: out
    ggg: out
    hhh: ccc fff iii
    iii: out";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT).unwrap(), "5")
    }

    #[test]
    fn problem2_test() {
        let input = "svr: aaa bbb
        aaa: fft
        fft: ccc
        bbb: tty
        tty: ccc
        ccc: ddd eee
        ddd: hub
        hub: fff
        eee: dac
        dac: fff
        fff: ggg hhh
        ggg: out
        hhh: out";
        assert_eq!(problem2(input).unwrap(), "2")
    }
}

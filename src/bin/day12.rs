use std::collections::{HashMap, HashSet};
use std::fs;

const FILENAME: &str = "input/day12.txt";

fn count_paths<'a>(
    graph: &HashMap<&str, HashSet<&'a str>>,
    visited: &mut HashMap<&'a str, u32>,
    current: &str,
    end: &str,
    start: &str,
    single: bool,
) -> u32 {
    graph
        .get(&current)
        .unwrap()
        .iter()
        .fold(0, |acc, next| match next {
            next if end.eq(*next) => acc + 1,
            next if start.eq(*next) => acc,
            next if *visited.get(next).unwrap_or(&0) >= 1
                && !next.chars().next().unwrap().is_ascii_uppercase()
                && single =>
            {
                acc
            }
            _ => {
                *visited.entry(next).or_insert(0) += 1;
                let count = count_paths(
                    graph,
                    visited,
                    next,
                    end,
                    start,
                    (*visited.get(next).unwrap() >= 2
                        && !next.chars().next().unwrap().is_ascii_uppercase())
                        || single,
                );
                *visited.get_mut(next).unwrap() -= 1;
                acc + count
            }
        })
}

fn main() {
    let content = fs::read_to_string(FILENAME).unwrap();

    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    content
        .lines()
        .map(|l| l.split_once("-").unwrap())
        .for_each(|(a, b)| {
            graph.entry(a).or_insert(HashSet::new()).insert(b);
            graph.entry(b).or_insert(HashSet::new()).insert(a);
        });

    let mut visited = HashMap::new();
    visited.insert("start", 1);
    println!(
        "{}",
        count_paths(&graph, &mut visited, "start", "end", "start", false)
    );
}

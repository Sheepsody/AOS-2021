use std::collections::{HashMap, HashSet};
use std::fs;

const FILENAME: &str = "input/day12.txt";

fn count_paths<'a>(
    graph: &HashMap<&str, HashSet<&'a str>>,
    visited: &mut HashSet<&'a str>,
    current: &str,
    end: &str,
) -> u32 {
    graph
        .get(&current)
        .unwrap()
        .iter()
        .fold(0, |acc, next| match next {
            next if end.eq(*next) => acc + 1,
            next if visited.contains(next)
                && !next.chars().next().unwrap().is_ascii_uppercase() =>
            {
                acc
            }
            _ => {
                visited.insert(next);
                let count = count_paths(graph, visited, next, end);
                visited.remove(next);
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

    let mut visited = HashSet::new();
    visited.insert("start");
    println!("{}", count_paths(&graph, &mut visited, "start", "end"));
}

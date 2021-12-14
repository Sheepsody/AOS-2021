use std::{collections::HashMap, fs};

const FILENAME: &str = "input/day14.txt";

fn main() {
    let content = fs::read_to_string(FILENAME).unwrap();
    let (polymer, rules_list) = content.split_once("\n\n").unwrap();

    let rules = rules_list
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .fold(HashMap::new(), |mut acc, (p, c)| {
            acc.insert(
                (p.chars().nth(0).unwrap(), p.chars().nth(1).unwrap()),
                c.parse::<char>().unwrap(),
            );
            acc
        });

    let last_char = polymer.chars().last().unwrap();

    let mut polymer_pairs: HashMap<(char, char), u64> =
        (0..polymer.len() - 1).fold(HashMap::new(), |mut acc, i| {
            *acc.entry((
                polymer.chars().nth(i).unwrap(),
                polymer.chars().nth(i + 1).unwrap(),
            ))
            .or_insert(0) += 1;
            acc
        });

    for _ in 0..40 {
        let mut new_pairs = HashMap::new();
        for (p, f) in polymer_pairs.iter() {
            if let Some(c) = rules.get(p) {
                *new_pairs.entry((p.0, c.clone())).or_insert(0) += f;
                *new_pairs.entry((c.clone(), p.1)).or_insert(0) += f;
            }
        }

        polymer_pairs = new_pairs;
    }
    polymer_pairs.insert((last_char, '$'), 1);

    // Get most frequent characters
    let frequencies: Vec<u64> = polymer_pairs
        .iter()
        .fold(HashMap::new(), |mut acc, (p, f)| {
            *acc.entry(p.0).or_insert(0) += f;
            acc
        })
        .iter()
        .map(|(_, i)| *i)
        .collect();

    let (min, max) = (
        frequencies.iter().min().unwrap(),
        frequencies.iter().max().unwrap(),
    );
    println!("{}", max - min);
}

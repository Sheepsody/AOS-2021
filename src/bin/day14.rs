use std::{collections::HashMap, fs};

const FILENAME: &str = "input/day14.txt";

fn main() {
    let content = fs::read_to_string(FILENAME).unwrap();
    let (polymer, rules_list) = content.split_once("\n\n").unwrap();

    let rules = rules_list
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .fold(HashMap::new(), |mut acc, (p, c)| {
            acc.insert(p, c.parse::<char>().unwrap());
            acc
        });

    let mut polymer = String::from(polymer);

    for _ in 0..10 {
        let mut insertions = vec![];
        for i in 0..polymer.len() - 1 {
            // Find points of insertion
            if let Some(c) = rules.get(polymer.get(i..=i + 1).unwrap()) {
                insertions.push((i + 1, c));
            }
        }
        insertions.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Insert in vector
        insertions.iter().enumerate().for_each(|(s, (i, c))| {
            polymer.insert(i + s, **c);
        });
    }

    // Get most frequent characters
    let frequencies: Vec<u32> = polymer
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
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

use std::fs;

fn main() {
    let content = fs::read_to_string("input/day8.txt").unwrap();

    let displays = content.lines();

    let signals = displays
        .clone()
        .map(|x| x.split("|").next().unwrap().split_whitespace());

    let outputs = displays
        .map(|x| x.split("|").last().unwrap().split_whitespace())
        .flatten();

    let unique_count = outputs
        .filter(|x| match x.len() {
            2 | 4 | 3 | 7 => true,
            _ => false,
        })
        .count();

    println!("{}", unique_count);
}

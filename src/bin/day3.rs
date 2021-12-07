use std::fs;

fn main() {
    let mut gamma = 0;
    let mut epsilon = 0;

    let content = fs::read_to_string("input/day3.txt").unwrap();

    let first = content.lines().next().unwrap();

    for i in 0..first.len() {
        let mut ones = 0;
        let mut zeros = 0;

        for line in content.lines() {
            let num = line.get(i..i + 1).unwrap().parse::<u32>().unwrap();
            match num {
                0 => zeros = zeros + 1,
                1 => ones = ones + 1,
                _ => panic!("Unvalid bit"),
            }
        }

        if ones > zeros {
            gamma = gamma * 2 + 1;
            epsilon = epsilon * 2;
        } else {
            gamma = gamma * 2;
            epsilon = epsilon * 2 + 1;
        };
    }

    println!("{}", gamma * epsilon)
}

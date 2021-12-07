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
            let num = line.get(i..i + 1).unwrap().parse::<u8>().unwrap();
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

    println!("{}", gamma * epsilon);

    let mut lines: Vec<usize> = (0..content.lines().count()).collect();
    let mut i = 0;

    while lines.len() > 1 {
        let mut ones = 0;
        let mut zeros = 0;

        for line in lines.clone() {
            let num = content
                .lines()
                .nth(line)
                .unwrap()
                .chars()
                .nth(i)
                .unwrap()
                .to_digit(10)
                .unwrap();

            match num {
                0 => zeros = zeros + 1,
                1 => ones = ones + 1,
                _ => panic!("Unvalid bit"),
            }
        }

        lines = lines
            .into_iter()
            .filter(|x| {
                content
                    .lines()
                    .nth(*x)
                    .unwrap()
                    .chars()
                    .nth(i)
                    .unwrap()
                    .to_digit(10)
                    .unwrap()
                    == (if ones >= zeros { 1 } else { 0 })
            })
            .collect();

        i = i + 1;
    }
    let oxy = content
        .lines()
        .nth(*lines.first().unwrap())
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .fold(0, |result, c| result * 2 + c);

    let mut lines: Vec<usize> = (0..content.lines().count()).collect();
    let mut i = 0;

    while lines.len() > 1 {
        let mut ones = 0;
        let mut zeros = 0;

        for line in lines.clone() {
            let num = content
                .lines()
                .nth(line)
                .unwrap()
                .get(i..i + 1)
                .unwrap()
                .parse::<u8>()
                .unwrap();

            match num {
                0 => zeros = zeros + 1,
                1 => ones = ones + 1,
                _ => panic!("Unvalid bit"),
            }
        }

        lines = lines
            .into_iter()
            .filter(|x| {
                content
                    .lines()
                    .nth(*x)
                    .unwrap()
                    .get(i..i + 1)
                    .unwrap()
                    .parse::<u8>()
                    .unwrap()
                    == (if zeros <= ones { 0 } else { 1 })
            })
            .collect();

        i = i + 1;
    }

    let co = content
        .lines()
        .nth(*lines.first().unwrap())
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .fold(0, |result, c| result * 2 + c);

    println!("{}", co);
    println!("{}", oxy);
    println!("{}", oxy * co)
}

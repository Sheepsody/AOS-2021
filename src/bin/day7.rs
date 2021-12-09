use std::fs;

fn main() {
    let mut content: Vec<i32> = fs::read_to_string("input/day7.txt")
        .unwrap()
        .replace("\n", "")
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    content.sort();

    // We just calculate the median
    let median = content[content.len() / 2];
    let moves = content.iter().map(|x| (x - median).abs()).sum::<i32>();

    println!("{}", moves);

    // If they change their cost
    let target: i32 = content.iter().sum::<i32>() / content.len() as i32;
    let fuel: i32 = (target..)
        .take(2)
        .map(|x| {
            content
                .iter()
                .map(|n| {
                    let d = (n - x).abs();
                    d * (d + 1) / 2
                })
                .sum()
        })
        .min()
        .unwrap();
    println!("{}", fuel)
}

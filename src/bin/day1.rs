use std::fs;

fn main() {
    let content = fs::read_to_string("input/day1.txt").unwrap();

    let mut count: u32 = 0;
    let mut prec: u32 = u32::MAX;

    for line in content.lines() {
        let current = line.parse::<u32>().unwrap();

        if current > prec {
            count = count + 1;
        }

        prec = current;
    }

    println!("{}", count);
}

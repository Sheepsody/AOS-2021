use std::fs;

fn main() {
    let content = fs::read_to_string("input/day2.txt").unwrap();

    let mut h: u32 = 0;
    let mut d: u32 = 0;

    for line in content.lines() {
        let mut pair = line.split_whitespace();

        let t = pair.next().unwrap();
        let c = pair.next().unwrap().parse::<u32>().unwrap();

        match t {
            "forward" => h = h + c,
            "down" => d = d + c,
            "up" => d = d - c,
            _ => panic!("Unrecognized token"),
        }
    }
    println!("H:{}, D:{}", h, d);
    println!("H*D:{}", h * d);
}

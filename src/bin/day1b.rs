use std::fs;

fn main() {
    let content = fs::read_to_string("input/day1.txt").unwrap();

    let mut count: u32 = 0;
    let (mut a, mut b, mut c) = (u32::MAX, u32::MAX, u32::MAX);

    for line in content.lines() {
        let d = line.parse::<u32>().unwrap();

        if !(a == u32::MAX) & !(b == u32::MAX) & !(c == u32::MAX) {
            if (b + c + d) > (a + b + c) {
                count = count + 1;
            }
        }

        a = b;
        b = c;
        c = d;
    }

    println!("{}", count);
}

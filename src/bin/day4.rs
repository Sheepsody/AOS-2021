use std::collections::BTreeMap;
use std::fs;
const ROW: u32 = 0b11111;
const COL: u32 = 0b100001000010000100001;

fn main() -> Result<(), ()> {
    let content = fs::read_to_string("input/day4.txt").unwrap();
    let mut lines: Vec<&str> = content.split("\n\n").collect();

    let header = lines.remove(0);

    let mut boards: Vec<(BTreeMap<u8, usize>, u32)> = lines
        .iter()
        .map(|b| {
            (
                b.split_whitespace()
                    .enumerate()
                    .map(|(i, n)| (n.parse().unwrap(), i))
                    .collect(),
                0,
            )
        })
        .collect();

    let (board, mark, num) = header
        .split(',')
        .map(|n| n.parse().unwrap())
        .find_map(|n| {
            boards.iter_mut().find_map(|(b, m)| {
                b.get(&n)
                    .map(|i| *m |= 1 << *i)
                    .filter(|_| (0..5).any(|i| *m >> i & COL == COL || *m >> (i * 5) & ROW == ROW))
                    .map(|_| (b.clone(), *m, n))
            })
        })
        .unwrap();

    println!(
        "{}",
        board
            .into_iter()
            .map(|(n, i)| (mark >> i & 1 ^ 1) * n as u32 * num as u32)
            .sum::<u32>()
    );

    Ok(())
}

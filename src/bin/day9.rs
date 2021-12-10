use std::fs;

fn main() {
    let content: Vec<Vec<u32>> = fs::read_to_string("input/day9.txt")
        .unwrap()
        .lines()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    let nlines = content.len() as i32;
    let ncols = content[0].len() as i32;

    let moves: Vec<(i32, i32)> = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let res = content
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter(|(j, n)| {
                    moves.iter().all(|(k, l)| {
                        if i as i32 + k >= 0
                            && i as i32 + k < nlines
                            && *j as i32 + l < ncols
                            && *j as i32 + l >= 0
                        {
                            **n < content[(i as i32 + k) as usize][(*j as i32 + l) as usize]
                        } else {
                            true
                        }
                    })
                })
                .fold(0, |sum, (_, n)| sum + n + 1)
        })
        .fold(0, |sum, n| sum + n);

    println!("{}", res);
}

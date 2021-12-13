use std::{fs, io::BufRead, slice::SliceIndex};

const FILENAME: &str = "input/day13.txt";
type Matrix = Vec<Vec<bool>>;

fn main() {
    let content = fs::read_to_string(FILENAME).unwrap();
    let (sgrid, sfolds) = content.split_once("\n\n").unwrap();

    let sgrid = sgrid.lines().map(|x| {
        x.split_once(",")
            .map(|c| (c.0.parse::<usize>().unwrap(), c.1.parse::<usize>().unwrap()))
            .unwrap()
    });

    let (maxx, maxy) = sgrid.clone().fold((0, 0), |mut acc, (x, y)| {
        if x > acc.0 {
            acc.0 = x
        };
        if y > acc.1 {
            acc.1 = y
        };
        acc
    });

    println!("{}, {}", maxx, maxy);

    let mut m: Matrix = vec![vec![false; maxy + 1]; maxx + 1];

    sgrid.for_each(|(x, y)| {
        m[x][y] = true;
    });

    sfolds
        .lines()
        .map(|l| {
            l.split_once("=")
                .map(|t| (t.0.chars().nth(11).unwrap(), t.1.parse::<usize>().unwrap()))
                .unwrap()
        })
        .enumerate()
        .for_each(|(i, (d, p))| {
            let (height, width) = (m.len(), m[0].len());
            match d {
                'x' => {
                    (p + 1..height).rev().for_each(|x| {
                        let ll = m.pop().unwrap();
                        ll.iter().enumerate().for_each(|(y, v)| {
                            m[height - x - 1][y] |= v;
                        })
                    });
                    m.pop();
                }
                'y' => {
                    m.iter_mut().for_each(|v| {
                        (p + 1..width).rev().for_each(|y| {
                            v[width - y - 1] |= v.pop().unwrap();
                        });
                        v.pop();
                    });
                }
                _ => panic!("Unexpected char!"),
            };

            let npoints = m
                .iter()
                .map(|v| v.iter().map(|p| *p as usize).sum::<usize>())
                .sum::<usize>();

            println!("Step: {}, Nb points: {}", i, npoints);
        });

    let (height, width) = (m.len(), m[0].len());
    println!(
        "{}",
        (0..width).fold(String::new(), |mut acc, y| {
            acc.push_str(
                format!(
                    "{}{}",
                    (0..height)
                        .map(|x| match m[x][y] {
                            true => "+",
                            false => " ",
                        })
                        .collect::<String>(),
                    "\n"
                )
                .as_str(),
            );
            acc
        })
    )
}

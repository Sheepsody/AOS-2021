use std::collections::HashMap;
use std::fs;

fn main() {
    let content = fs::read_to_string("input/day5.txt").unwrap();

    let mut overlapps = 0;
    let mut points = HashMap::new();

    let lines = content.lines().map(|l| {
        let mut coords: Vec<i32> = l
            .split(" -> ")
            .map(|p| p.split(",").map(|x| x.parse::<i32>().unwrap()))
            .flatten()
            .collect();

        (
            coords.pop().unwrap(),
            coords.pop().unwrap(),
            coords.pop().unwrap(),
            coords.pop().unwrap(),
        )
    });

    lines.for_each(|(x1, y1, x2, y2)| {
        let mut add_point = |x, y| {
            match points.get(&(x, y)) {
                None => {
                    points.insert((x, y), 0);
                }
                Some(1) => overlapps += 1,
                _ => (),
            }
            if let Some(x) = points.get_mut(&(x, y)) {
                *x += 1;
            }
        };

        if x1 == x2 {
            (y1.min(y2)..=y1.max(y2)).for_each(|y| add_point(x1, y))
        } else if y1 == y2 {
            (x1.min(x2)..=x1.max(x2)).for_each(|x| add_point(x, y1))
        } else {
            (0..=(x2 - x1)).for_each(|i| {
                add_point(
                    x1 + i,
                    y1 + i * {
                        if (y2 - y1) * (x2 - x1) > 0 {
                            1
                        } else {
                            -1
                        }
                    },
                )
            })
        }
    });

    println!("{}", overlapps);
}

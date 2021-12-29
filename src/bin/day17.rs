// x <- x - 1 * sign(x) if x != 0
// y <- y - 1
// Goal: highest position in y it can reach

use std::fs;

use itertools::Itertools;

fn main() {
    let content = fs::read_to_string("./input/day17.txt").unwrap();

    let (xmin, xmax, ymin, ymax) = content
        .trim()
        .trim_start_matches("target area: x=")
        .split(", y=")
        .map(|l| l.split(".."))
        .flatten()
        .map(|x| x.parse::<i32>().unwrap())
        .collect_tuple()
        .unwrap();

    let vmax = ymin.abs() - 1;

    println!("{}", vmax * (vmax + 1) / 2)
}

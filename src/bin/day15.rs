use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;

const FILENAME: &str = "input/day15.txt";
const MOVES: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

#[derive(Copy, Clone)]
struct Cell {
    l: (i32, i32),
    d: u32,
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.d == other.d
    }
}

impl Eq for Cell {}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.d.partial_cmp(&self.d)
    }
}

impl Ord for Cell {
    fn cmp(&self, other: &Self) -> Ordering {
        other.d.cmp(&self.d)
    }
}

fn main() {
    let mut points: HashMap<(i32, i32), u32> = HashMap::new();
    fs::read_to_string(FILENAME)
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(i, l)| {
            l.chars().enumerate().for_each(|(j, c)| {
                points.insert((i as i32, j as i32), c.to_digit(10).unwrap());
            });
        });

    let mut distances: HashMap<(i32, i32), u32> = HashMap::new();
    let mut queue = BinaryHeap::new();

    distances.insert((0, 0), 0);
    queue.push(Cell { l: (0, 0), d: 0 });

    while let Some(Cell { l, d }) = queue.pop() {
        MOVES
            .iter()
            .map(|&(x, y)| (l.0 as i32 + x, l.1 as i32 + y))
            .for_each(|(x, y)| {
                if let Some(w) = points.get(&(x, y)) {
                    let new_d = d + w;
                    if distances.get(&(x, y)).map_or(true, |&cur_d| new_d < cur_d) {
                        distances.insert((x, y), new_d);
                        queue.push(Cell {
                            l: (x, y),
                            d: new_d,
                        });
                    }
                }
            });
    }

    let x_max = points.iter().map(|((x, _), _)| x).max().unwrap();
    let y_max = points.iter().map(|((_, y), _)| y).max().unwrap();
    println!("{}", distances.get(&(*x_max, *y_max)).unwrap())
}

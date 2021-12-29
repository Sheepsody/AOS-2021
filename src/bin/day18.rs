use std::fmt;
use std::fs;
use std::{ops::Add, process::Output};

use itertools::Itertools;

enum Element {
    Number(u8),
    Pair(Box<SnailfishNumber>),
}

struct SnailfishNumber {
    left: Element,
    right: Element,
    depth: usize,
}

impl Add for SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, other: Self) -> SnailfishNumber {
        SnailfishNumber {
            left: Element::Pair(Box::new(self)),
            right: Element::Pair(Box::new(other)),
            depth: 0,
        }
    }
}

impl SnailfishNumber {
    fn from_string(s: &str, depth: usize) -> SnailfishNumber {
        let mut p = 0;
        for i in 1..s.len() {
            match s.chars().nth(i).unwrap() {
                '[' => p += 1,
                ']' => p -= 1,
                ',' if p == 0 => {
                    let (left, right) = vec![&s[1..i], &s[i + 1..s.len() - 1]]
                        .iter()
                        .map(|x| {
                            if let Ok(n) = x.trim().parse::<u8>() {
                                Element::Number(n)
                            } else {
                                Element::Pair(Box::new(SnailfishNumber::from_string(x, depth + 1)))
                            }
                        })
                        .collect_tuple()
                        .unwrap();

                    return SnailfishNumber { left, right, depth };
                }
                _ => (),
            }
        }
        panic!("Could not parse Snailfish number")
    }
}

fn main() {
    let nums: SnailfishNumber = fs::read_to_string("./input/day18.test.txt")
        .unwrap()
        .lines()
        .map(|s| SnailfishNumber::from_string(s, 0))
        .reduce(|a, e| a + e)
        .unwrap();
}

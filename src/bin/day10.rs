use std::fs;

#[derive(Copy, Clone, PartialEq, Debug)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Side {
    Left,
    Right,
}

fn get_score(v: Vec<(Bracket, Side)>) -> u32 {
    v.iter()
        .map(|(b, _)| match b {
            Bracket::Round => 1,
            Bracket::Square => 2,
            Bracket::Curly => 3,
            Bracket::Angle => 4,
        })
        .rev()
        .fold(0, |t, n| t * 5 + n)
}

fn parse_line(line: Vec<(Bracket, Side)>) -> Option<Vec<(Bracket, Side)>> {
    line.iter().fold(
        Some(vec![]),
        |mut res: Option<Vec<(Bracket, Side)>>, (bracket, side)| {
            if let Some(ref mut stack) = res {
                return match side {
                    Side::Left => {
                        stack.push((*bracket, *side));
                        Some(stack.to_vec())
                    }
                    Side::Right => match stack.pop() {
                        Some((bb, ss)) => {
                            if bb != *bracket || ss == *side {
                                return None;
                            }
                            Some(stack.to_vec())
                        }
                        None => None,
                    },
                };
            } else {
                res
            }
        },
    )
}

fn get_median(mut v: Vec<u32>) -> u32 {
    v.sort();
    let pos = v.len() / 2;
    v[pos]
}

fn main() {
    let res: Vec<u32> = fs::read_to_string("input/day10.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| {
                    let bracket = match c {
                        ')' | '(' => Bracket::Round,
                        ']' | '[' => Bracket::Square,
                        '}' | '{' => Bracket::Curly,
                        '>' | '<' => Bracket::Angle,
                        _ => panic!("Unrecognized token {}", c),
                    };
                    let side = match c {
                        '(' | '{' | '[' | '<' => Side::Left,
                        ')' | '}' | ']' | '>' => Side::Right,
                        _ => panic!("Unrecognized token {}", c),
                    };
                    (bracket, side)
                })
                .collect()
        })
        .filter_map(|l| parse_line(l))
        .filter(|v| !v.is_empty())
        .map(|v| get_score(v))
        .collect();

    println!("{}", get_median(res));
}

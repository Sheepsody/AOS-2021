use std::fs;

#[derive(Copy, Clone, PartialEq)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

#[derive(Copy, Clone, PartialEq)]
enum Line {
    Corrupted,
    Ok,
}

#[derive(Copy, Clone, PartialEq)]
enum Side {
    Left,
    Right,
}

fn get_score(b: Bracket) -> u32 {
    match b {
        Bracket::Round => 3,
        Bracket::Square => 57,
        Bracket::Curly => 1197,
        Bracket::Angle => 25137,
    }
}

fn check_line(line: Vec<(Bracket, Side)>) -> (Line, u32) {
    let res = line.iter().fold(
        Ok(vec![]),
        |mut res: Result<Vec<(Bracket, Side)>, u32>, (bracket, side)| {
            if let Ok(ref mut stack) = res {
                return match side {
                    Side::Left => {
                        stack.push((*bracket, *side));
                        Ok(stack.to_vec())
                    }
                    Side::Right => match stack.pop() {
                        Some((bb, ss)) => {
                            if bb != *bracket {
                                return Err(get_score(*bracket));
                            }
                            if ss == *side {
                                return Err(get_score(*bracket));
                            }
                            Ok(stack.to_vec())
                        }
                        None => Err(get_score(*bracket)),
                    },
                };
            } else {
                res
            }
        },
    );

    match res {
        Err(x) => (Line::Corrupted, x),
        _ => (Line::Ok, 0),
    }
}

fn main() {
    let res: u32 = fs::read_to_string("input/day10.test.txt")
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
        .filter_map(|l| match check_line(l) {
            (Line::Corrupted, x) => Some(x),
            _ => None,
        })
        .sum();

    println!("{}", res);
}

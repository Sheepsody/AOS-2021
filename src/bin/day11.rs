use std::fs;

const NEXT: [(isize, isize); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];
type Matrix = Vec<Vec<u32>>;

fn get_matrix() -> Matrix {
    fs::read_to_string("input/day11.txt")
        .unwrap()
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn flash(m: &mut Matrix, i: usize, j: usize) -> u32 {
    m[i][j] = 0;
    NEXT.iter()
        .map(|(ii, jj)| ((i as isize + ii) as usize, (j as isize + jj) as usize))
        .fold(1, |acc, (i, j)| {
            match m.get_mut(i).and_then(|row| row.get_mut(j)) {
                Some(n) if *n > 0 => {
                    *n += 1;
                    acc + (*n > 9).then(|| flash(m, i, j)).unwrap_or(0)
                }
                _ => acc,
            }
        })
}

fn main() {
    let mut m = get_matrix();

    let height = m.len();
    let width = m[0].len();

    let flashes_count = (1..)
        .find(|_| {
            m.iter_mut()
                .for_each(|row| row.iter_mut().for_each(|n| *n += 1));
            (0..height)
                .map(|i| {
                    (0..width)
                        .map(|j| if m[i][j] > 9 { flash(&mut m, i, j) } else { 0 })
                        .sum::<u32>()
                })
                .sum::<u32>()
                == (width * height) as u32
        })
        .unwrap();

    println!("{}", flashes_count)
}

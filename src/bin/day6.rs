use std::collections::VecDeque;
use std::fs;

fn main() {
    let mut init = fs::read_to_string("input/day6.txt")
        .unwrap()
        .replace("\n", "")
        .split(',')
        .fold([0; 9], |mut map, n| {
            println!("{}", n);
            if let Ok(e) = n.parse::<usize>() {
                map[e] += 1;
            }
            map
        });

    let mut queue = VecDeque::from(init);

    for i in 0..256 {
        let first = queue.pop_front().unwrap();
        if let Some(e) = queue.get_mut(6) {
            *e += first;
        }
        queue.push_back(first);
        println!("after {}, {}", i + 1, queue.get(0).unwrap())
    }

    let count: usize = queue.iter().sum();

    println!("{}", count);
}

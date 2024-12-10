use std::{
    collections::HashSet, fs::File, io::{BufRead, BufReader}
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let reader = BufReader::new(File::open(args[1].clone()).expect("open failed"));

    let mut map: Vec<Vec<char>> = vec![];
    let mut zeroes = vec![];

    for (i, line) in reader.lines().enumerate() {
        let chars: Vec<char> = line.expect("lines failed").chars().collect();

        for (j, c) in chars.iter().enumerate() {
            if *c == '0' {
                zeroes.push((i, j));
            }
        }

        map.push(chars);
    }

    let mut sum = 0;

    for zero in zeroes.iter() {
        sum += dfs(&map, *zero, 0)
            .iter()
            .collect::<HashSet<_>>()
            .iter()
            .collect::<Vec<_>>()
            .len();
    }

    println!("{}", sum);
}

fn dfs(map: &Vec<Vec<char>>, start: (usize, usize), target: u32) -> Vec<(usize, usize)> {
    // bounds check
    if start.0 > (map.len() - 1) || start.1 > (map[start.0].len() - 1) {
        return vec![];
    }

    let c = map[start.0][start.1].to_digit(10).expect("not digit");

    if c == target && c == 9 {
        return vec![start];
    }

    if c != target {
        return vec![];
    }

    let mut up = match start.0.checked_sub(1) {
        Some(y) => dfs(map, (y, start.1), target + 1), // UP
        None => vec![],
    };

    let mut right = dfs(map, (start.0, start.1 + 1), target + 1); // RIGHT
    let mut down = dfs(map, (start.0 + 1, start.1), target + 1); // DOWN
    let mut left = match start.1.checked_sub(1) {
        Some(x) => dfs(map, (start.0, x), target + 1), // LEFT
        None => vec![],
    };

    up.append(&mut right);
    up.append(&mut down);
    up.append(&mut left);
    return up;
}

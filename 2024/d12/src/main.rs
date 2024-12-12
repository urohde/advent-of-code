use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let reader = BufReader::new(File::open(args[1].clone()).expect("open failed"));

    let map: Vec<Vec<char>> = reader
        .lines()
        .map(|l| l.expect("lines failed").chars().collect())
        .collect();

    for row in &map {
        for c in row {
            print!("{}", c);
        }
        println!();
    }

    let mut visited: Vec<bool> = vec![false; map.len() * map[0].len()];
    let width = map.len() - 1;

    let mut current_area: Option<char> = None;
    let mut sum = 0;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if visited[width * y + x] {
                continue;
            }

            if current_area.is_none() {
                // find new area
                for y_1 in 0..map.len() {
                    for x_1 in 0..map[0].len() {
                        let c = map[y_1][x_1];
                        if visited[width * y_1 + x_1] {
                            continue;
                        }
                        current_area = Some(c);
                        break;
                    }
                    if current_area.is_some() {
                        break;
                    }
                }
            }

            // count perimiter
            let mut perimiter_count = 0;
            let mut area_size = 0;
            for y_1 in 0..map.len() {
                for x_1 in 0..map[0].len() {
                    let c = map[y_1][x_1];
                    if visited[width * y_1 + x_1] || c != current_area.expect("no area") {
                        continue;
                    }
                    visited[width * y_1 + x_1] = true;
                    area_size += 1;
                    perimiter_count +=
                        count_perimiter(&y, &x, current_area.expect("no area"), &map);
                    print!("P = {} ", perimiter_count);
                }
            }

            // calculate and add price
            println!("{} * {}", area_size, perimiter_count);
            sum += area_size * perimiter_count;
        }
    }

    println!("{}", sum);
}

fn count_perimiter(y: &usize, x: &usize, area: char, map: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    match y.checked_sub(1) {
        // UP
        Some(y) => {
            if map[y][*x] != area {
                sum += 1;
            }
        }
        None => {
            sum += 1;
        }
    }

    match *y < (map.len() - 1) {
        true => {
            if map[*y + 1][*x] != area {
                // DOWN
                sum += 1;
            }
        }
        false => {
            sum += 1;
        }
    }

    match *x < (map[0].len() - 1) {
        true => {
            if *x < (map[0].len() - 1) && map[*y][*x + 1] != area {
                // RIGHT
                sum += 1;
            }
        }
        false => {
            sum += 1;
        }
    }

    match x.checked_sub(1) {
        // LEFT
        Some(x) => {
            if map[*y][x] != area {
                sum += 1;
            }
        }
        None => {
            sum += 1;
        }
    };
    return sum;
}

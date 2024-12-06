use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Coordinate = (usize, usize);

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn turn(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn next_coordinate(c: Coordinate, d: &Direction) -> Option<Coordinate> {
    match d {
        Direction::Up => match c.0.checked_sub(1) {
            Some(y) => Some((y, c.1)),
            None => None,
        },
        Direction::Right => Some((c.0, c.1 + 1)),
        Direction::Down => Some((c.0 + 1, c.1)),
        Direction::Left => match c.1.checked_sub(1) {
            Some(x) => Some((c.0, x)),
            None => None,
        },
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let reader = BufReader::new(File::open(args[1].clone()).expect("open failed"));

    let mut map: Vec<Vec<char>> = vec![];
    let mut guard: Option<Coordinate> = None;

    for (i, line) in reader.lines().enumerate() {
        let line = line.expect("lines failed");

        let mut current = vec![];
        for (j, c) in line.chars().enumerate() {
            if c == '^' {
                guard = Some((i, j));
            }
            current.push(c);
        }
        map.push(current);
    }

    let max_coordinates = (map.len() - 1, map[0].len() - 1);
    let mut guard = guard.expect("didn't find guard");
    let mut direction = Direction::Up;
    let mut visited = vec![0; map.len() * map[0].len()];

    loop {
        let c = match next_coordinate(guard, &direction) {
            Some(c) => {
                if c.0 > max_coordinates.0 || c.1 > max_coordinates.1 {
                    break;
                }
                c // naked return
            }
            None => {
                break;
            }
        };

        if map[c.0][c.1] == '#' {
            direction = turn(direction);
            continue;
        }

        guard = c;
        visited[max_coordinates.0 * c.0 + c.1] = 1;
    }

    println!("{}", visited.iter().sum::<i32>());
}

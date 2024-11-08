use std::{fs::*, io::*, path::Path};

struct Tile {
    heat_loss: i32,
    render: char,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.render)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = read_lines(&args[1]).unwrap();

    let mut grid: Vec<Vec<Tile>> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        grid.push(
            line.chars()
                .map(|c| Tile {
                    heat_loss: c.to_digit(10).unwrap().try_into().unwrap(),
                    render: c,
                })
                .collect(),
        );
    }

    for row in &grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }

    let mut position = (0, 0);
    let goal = (grid.len() as i32, grid[0].len() as i32);
    let mut sum_heat_loss = 0;

    let mut same_direction = (0, 0, 0, 0);

let mut moves = 0;
    
    while position != goal {
        let mut min_heat_loss = std::i32::MAX;
        let mut next_position = position;

        for direction in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_position = (position.0 + direction.0, position.1 + direction.1);

            if is_out_of_bounds(&goal, &next_position)
                || is_backwards(direction, &same_direction)
                || is_over_straight_line_limit(&position, &new_position, &same_direction)
                || !grid[new_position.0 as usize][new_position.1 as usize].render.is_numeric()
            {
                continue;
            }

            let heat_loss = grid[next_position.0 as usize][next_position.1 as usize].heat_loss;
            if heat_loss < min_heat_loss {
                min_heat_loss = heat_loss;
                next_position = new_position;
            }
        }

        sum_heat_loss += min_heat_loss;
        let direction = (next_position.0 - position.0, next_position.1 - position.1);
        let mut moved = 'A';

        if direction.0 == 1 {
            if same_direction.0 == 0 {
                same_direction = (0, 0, 0, 0);
            }
            same_direction.0 += 1;
        } else if direction.0 == -1 {
            if same_direction.1 == 0 {
                same_direction = (0, 0, 0, 0);
            }
            same_direction.1 += 1;
            moved = '>';
        } else if direction.1 == 1 {
            if same_direction.2 == 0 {
                same_direction = (0, 0, 0, 0);
            }
            same_direction.2 += 1;
            moved = 'V';
        } else if direction.1 == -1 {
            if same_direction.3 == 0 {
                same_direction = (0, 0, 0, 0);
            }
            same_direction.3 += 1;
            moved = '<';
        }

        grid[position.0 as usize][position.1 as usize].render = moved;
        position = next_position;
        moves += 1;

        if moves % 10 == 0 {
            println!("-----");
            for row in &grid {
            for c in row {
                print!("{}", c);
            }
            println!();
            }
        }
    }

    println!("-----");
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!();
    }
    println!("Minimum heat loss: {}", sum_heat_loss);
}

fn is_out_of_bounds(bounds: &(i32, i32), position: &(i32, i32)) -> bool {
    return position.0 < 0 || position.0 >= bounds.0 || position.1 < 0 || position.1 >= bounds.1;
}

fn is_backwards(direction: &(i32, i32), same_direction: &(i32, i32, i32, i32)) -> bool {
    return direction.0 == -same_direction.0
        && direction.1 == -same_direction.1
        && direction.0 == -same_direction.2
        && direction.1 == -same_direction.3;
}

fn is_over_straight_line_limit(
    position: &(i32, i32),
    new_position: &(i32, i32),
    same_direction: &(i32, i32, i32, i32),
) -> bool {
    let direction = (new_position.0 - position.0, new_position.1 - position.1);
    let same_direction = (
        same_direction.0 + direction.0,
        same_direction.1 + direction.1,
        same_direction.2 + direction.0,
        same_direction.3 + direction.1,
    );
    return same_direction.0 >= 3
        || same_direction.1 >= 3
        || same_direction.2 >= 3
        || same_direction.3 >= 3;
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

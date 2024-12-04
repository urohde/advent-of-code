use std::{fs::*, io::*, path::Path};

struct Rock {
    row: usize,
    potential: usize,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = read_lines(&args[1]).unwrap();

    let mut rocks: Vec<Rock> = Vec::new();

    let mut potentials: Vec<usize> = Vec::new();
    let mut row_count = 0;
    for (row, line) in lines.enumerate() {
        let line = line.unwrap();
        row_count += 1;

        if potentials.len() == 0 {
            potentials = vec![0; line.len()];
        }

        for (j, c) in line.chars().enumerate() {
            match c {
                'O' => rocks.push(Rock {
                    row,
                    potential: potentials[j],
                }),
                '#' => potentials[j] = 0,
                _ => potentials[j] += 1,
            }
        }
    }

    let mut sum = 0;
    for rock in rocks {
        sum += row_count - rock.row + rock.potential;
    }
    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

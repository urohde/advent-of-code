use std::{fs::*, io::*, path::Path};

#[derive(Debug)]
struct Galaxy {
    name: String,
    location: (usize, usize),
}

#[derive(Debug)]
struct Distance {
    from: String,
    to: String,
    distance: usize,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Reading file {}", args[1]);

    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut space: Vec<Vec<String>> = Vec::new();
    let mut no_expand_columns: Vec<usize> = Vec::new();
    let mut expand_rows: Vec<usize> = Vec::new();

    if let Ok(lines) = read_lines(&args[1]) {
        for (i, line) in lines.into_iter().enumerate() {
            if let Ok(l) = line {
                space.push(Vec::new());
                let mut empty_row = true;

                for (j, word) in l.split("").enumerate() {
                    match word {
                        "#" => {
                            let y = i;
                            let x = j - 1;
                            galaxies.push(Galaxy {
                                name: (galaxies.len() + 1).to_string(),
                                location: (x, y),
                            });
                            space[i].push(galaxies.len().to_string());
                            empty_row = false;
                            if !no_expand_columns.contains(&x) {
                                no_expand_columns.push(x);
                            }
                        }
                        "." => {
                            space[i].push(".".to_string());
                        }
                        _ => {}
                    };
                }
                if empty_row {
                    expand_rows.push(i);
                }
            }
        }
    }

    let expand_columns: Vec<usize> = (0..space[0].len())
        .filter(|x| !no_expand_columns.contains(x))
        .collect();

    println!("expand_columns: {:?}", expand_columns);
    println!("expand_rows: {:?}", expand_rows);

    println!("--- initial ---");

    print_galaxies(&galaxies);
    print_space(&space);
    println!("--- expansion ---");

    // expand space
    for (i, row) in expand_rows.iter().enumerate() {
        let new_row = space[*row + i].clone();
        space.insert(*row + i, new_row)
    }

    for row in space.iter_mut() {
        for (j, col) in expand_columns.iter().enumerate() {
            row.insert(*col + j, ".".to_string());
        }
    }

    for galaxy in galaxies.iter_mut() {
        for (j, col) in expand_columns.iter().enumerate() {
            if galaxy.location.0 > *col {
                galaxy.location.0 += j;
            }
        }

        for row in expand_rows.iter() {
            if galaxy.location.1 > *row {
                galaxy.location.1 += 1;
            }
        }
    }

    // reconstruct space
    let mut new_space = vec![vec!["".to_string(); space[0].len() + 1]; space.len() + 1];
    for row in 0..space.len() {
        for col in 0..space[0].len() {
            match galaxies.iter().find(|g| g.location.0 == col && g.location.1 == row) {
                Some(g) => {
                    new_space[row][col] = g.name.clone();
                }
                None => {
                    new_space[row][col] = ".".to_string();
                }
            }
        }
    }

    print_galaxies(&galaxies);
    print_space(&space);
    println!("--- new space ---");
    print_space(&new_space);

    unimplemented!();

    println!("--- distances ---");

    let mut distances: Vec<Distance> = Vec::new();
    for g1 in galaxies.iter() {
        for g2 in galaxies.iter() {
            if g2.name == g1.name {
                continue;
            }
            if distances
                .iter()
                .find(|d| d.from == g2.name && d.to == g1.name)
                .is_none()
            {
                distances.push(Distance {
                    from: g1.name.clone(),
                    to: g2.name.clone(),
                    distance: g1.location.0.abs_diff(g2.location.0)
                        + g1.location.1.abs_diff(g2.location.1),
                })
            }
        }
    }

    for d in distances.iter() {
        print!("{} -> {} = {} \n", d.from, d.to, d.distance);
    }

    let sum = distances.iter().fold(0, |acc, d| acc + d.distance);
    println!("--- sum ---");
    println!("pairs: {}", distances.len());
    println!("sum: {}", sum);
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn print_space(space: &Vec<Vec<String>>) {
    for line in space {
        for word in line {
            print!("{}", word);
        }
        print!("\n");
    }
}

fn print_galaxies(galaxies: &Vec<Galaxy>) {
    for galaxy in galaxies {
        println!(
            "Galaxy {} ({},{})",
            galaxy.name, galaxy.location.0, galaxy.location.1
        );
    }
}

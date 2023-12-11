use std::{fs::*, io::*, path::Path};

#[derive(Debug)]
struct Galaxy {
    name: String,
    location: (usize, usize),
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
    for row in expand_rows {
        let new_row = space[row].clone();
        space.insert(row, new_row)
    }

    for row in space.iter_mut() {
        for col in expand_columns.iter() {
            row.insert(*col, ".".to_string());
        }
    }

    print_galaxies(&galaxies);
    print_space(&space);
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

use std::{fs::*, io::*, path::Path};

#[derive(Debug, Clone)]
struct Dimension {
    space: Vec<Vec<String>>,
    galaxies: Vec<Galaxy>,
    expand_rows: Vec<usize>,
    expand_columns: Vec<usize>,
}

impl Dimension {
    fn print_space(&self) {
        println!("space: {}x{}", self.space[0].len(), self.space.len());
        for line in self.space.iter() {
            for word in line {
                print!("{}", word);
            }
            print!("\n");
        }
    }

    fn print_galaxies(&self) {
        println!("galaxies: {}", self.galaxies.len());
        for galaxy in self.galaxies.iter() {
            println!(
                "Galaxy {} ({},{})",
                galaxy.name, galaxy.location.0, galaxy.location.1
            );
        }
    }

    fn print_expand(&self) {
        println!("expand_columns: {:?}", self.expand_columns);
        println!("expand_rows: {:?}", self.expand_rows);
    }

    fn print(&self) {
        self.print_space();
        self.print_galaxies();
        self.print_expand();
    }

    fn expand(&mut self, times: usize) {
        self.expand_space();
        self.expand_galaxies(times);
    }

    fn expand_space(&mut self) {
        for (i, row) in self.expand_rows.iter().enumerate() {
            let new_row = self.space[*row + i]
                .clone()
                .iter()
                .map(|_| "?".to_string())
                .collect();

            self.space.insert(*row + i, new_row)
        }

        for row in self.space.iter_mut() {
            for (j, col) in self.expand_columns.iter().enumerate() {
                row.insert(*col + j, "?".to_string());
            }
        }
    }

    fn expand_galaxies(&mut self, times: usize) {
        for galaxy in self.galaxies.iter_mut() {
            let mut col_to_add = 0;
            for col in self.expand_columns.iter() {
                if galaxy.location.0 > *col {
                    col_to_add += 1;
                }
            }

            let mut row_to_add = 0;
            for row in self.expand_rows.iter() {
                if galaxy.location.1 > *row {
                    row_to_add += 1;
                }
            }

            galaxy.location.0 += col_to_add * (times - 1);
            galaxy.location.1 += row_to_add * (times - 1);
        }
    }

    fn get_distances(&self) -> Vec<usize> {
        let mut distances: Vec<usize> = Vec::new();
        for g1 in self.galaxies.iter() {
            for g2 in self.galaxies.iter() {
                if g2.name == g1.name || g1.name > g2.name {
                    continue;
                }
                distances.push(
                    g1.location.0.abs_diff(g2.location.0) + g1.location.1.abs_diff(g2.location.1),
                )
            }
        }

        return distances;
    }
}

#[derive(Debug, Clone)]
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
    println!("Expanding {} times", args[2]);

    let input_dimension = parse_space(read_lines(&args[1]).unwrap());
    let times = args[2].parse::<usize>().unwrap();

    println!("Found {} galaxies", input_dimension.galaxies.len());

    let mut new_dimension = input_dimension.clone();
    new_dimension.expand_galaxies(times);

    let distances = new_dimension.get_distances();
    print_distances(distances);
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn parse_space(lines: Lines<BufReader<File>>) -> Dimension {
    let mut galaxies: Vec<Galaxy> = Vec::new();
    let mut space: Vec<Vec<String>> = Vec::new();

    let mut no_expand_columns: Vec<usize> = Vec::new();
    let mut expand_rows: Vec<usize> = Vec::new();

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

    let expand_columns: Vec<usize> = (0..space[0].len())
        .filter(|x| !no_expand_columns.contains(x))
        .collect();

    return Dimension {
        space,
        galaxies,
        expand_rows,
        expand_columns,
    };
}

fn compare_dimensions(a: &Dimension, b: &Dimension) {
    println!("--- comparison ---");

    let mut same_galaxies = true;
    for i in 0..a.galaxies.len() {
        let g1 = &a.galaxies[i];
        let g2 = &b.galaxies[i];
        if g1.location != g2.location {
            same_galaxies = false;
            println!(
                "Galaxy {} ({},{}) != ({},{})",
                g1.name, g1.location.0, g1.location.1, g2.location.0, g2.location.1
            );
        }
    }

    if same_galaxies {
        println!("Galaxies are the same");
    }

    let mut same = true;
    let mut compare_space = a.space.clone();

    for i in 0..a.space.len() {
        for j in 0..a.space[i].len() {
            let w1 = &a.space[i][j];
            let w2 = &b.space[i][j];
            if w1 != w2 {
                same = false;
                compare_space[i][j] = "X".to_string();
            }
        }
    }

    if !same {
        println!("--- compare space ---");
        for line in compare_space.iter() {
            for word in line {
                print!("{}", word);
            }
            print!("\n");
        }
    }
}

fn print_distances(distances: Vec<usize>) {
    // for d in distances.iter() {
    //     print!("{} -> {} = {} \n", d.from, d.to, d.distance);
    // }

    let sum = distances.iter().fold(0, |acc, d| acc + d);
    println!("Pairs of galaxies checked: {}", distances.len());
    println!("Sum of distance between galaxies: {}", sum);
}

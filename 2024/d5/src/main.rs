use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let reader = BufReader::new(File::open(args[1].clone()).expect("open failed"));

    let mut rules = HashMap::new();
    let mut finished_rules = false;

    let mut sum = 0;

    'lines: for read in reader.lines() {
        let line = read.expect("lines failed");

        if line.trim().is_empty() && !finished_rules {
            finished_rules = true;
            continue 'lines;
        }
        match finished_rules {
            false => {
                let parts: Vec<String> = line.split('|').map(|s| s.to_string()).collect();
                rules
                    .entry(parts[0].clone())
                    .and_modify(|e: &mut Vec<String>| e.push(parts[1].clone()))
                    .or_insert(vec![parts[1].clone()]);
                continue 'lines;
            }
            true => {
                let parts: Vec<&str> = line.split(',').collect();
                for part in parts.iter() {
                    let rule = rules.get(*part);
                    match rule {
                        Some(r) => {
                            'rule_checker: for p in parts.iter() {
                                if p == part {
                                    break 'rule_checker;
                                }
                                if r.contains(&p.to_string()) {
                                    continue 'lines;
                                }
                            }
                        }
                        None => ()
                    }
                }
                sum += parts[parts.len() / 2]
                    .parse::<i32>()
                    .expect("failed to parse number");
            }
        }
    }

    println!("{}", sum);
}

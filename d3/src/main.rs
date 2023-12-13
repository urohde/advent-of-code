use std::{collections::HashSet, fs::*, io::*, path::Path};

#[derive(Debug)]
struct Part {
    number: i32,
    start: usize,
    end: usize,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = &args[1];

    let mut lines = read_lines(filename).unwrap();

    let mut parts: HashSet<i32> = HashSet::new();

    let mut last = String::new();
    let mut current = lines.next().unwrap().unwrap();
    let mut next = lines.next();
    let mut last_check: Vec<usize> = Vec::new();

    loop {
        let current_check: Vec<usize> = find_check(current.chars().collect());
        let next_check: Vec<usize> = match &next {
            Some(Ok(s)) => find_check(s.chars().collect()),
            _ => Vec::new(),
        };

        let mut lc_check = current_check.clone();
        lc_check.extend(last_check.clone());
        let mut lcn_check = lc_check.clone();
        lcn_check.extend(next_check.clone());

        let last_parts = find_parts(last.chars().collect(), &current_check);
        let current_parts = find_parts(current.chars().collect(), &lcn_check);

        parts.extend(last_parts.iter().map(|part| part.number));
        parts.extend(current_parts.iter().map(|part| part.number));

        last = current;
        last_check = current_check;
        current = match next {
            Some(Ok(s)) => s,
            Some(Err(_)) => break,
            None => break,
        };
        next = lines.next();
    }

    println!("{:?}", parts);
    println!("{}", parts.iter().sum::<i32>());
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn find_check(line: Vec<char>) -> Vec<usize> {
    let mut current_check: Vec<usize> = Vec::new();
    for (j, c) in line.iter().enumerate() {
        if c.is_digit(10) || *c == '.' {
            continue;
        }

        if j > 0 {
            current_check.push(j - 1);
        }

        current_check.push(j);

        if 1 < line.len() {
            current_check.push(j + 1);
        }
    }
    return current_check;
}

fn find_parts(line: Vec<char>, indexes: &Vec<usize>) -> Vec<Part> {
    let mut parts: Vec<Part> = Vec::new();
    for i in indexes {
        let i = *i;
        if line[i].is_digit(10) {
            if parts
                .iter()
                .find(|part| i >= part.start && i <= part.end)
                .is_none()
            {
                parts.push(find_part(&line, i));
            }
        }
    }
    return parts;
}

fn find_part(line: &Vec<char>, found: usize) -> Part {
    let mut start = found;
    let mut end = found;

    let mut found_start = false;
    let mut found_end = false;

    loop {
        if found_start && found_end {
            break;
        }

        if start > 0 && line[start - 1].is_digit(10) {
            start -= 1;
        } else {
            found_start = true;
        }

        if end < line.len() && line[end].is_digit(10) {
            end += 1;
        } else {
            found_end = true;
        }
    }

    let part_number: i32 = line[start..end].iter().collect::<String>().parse().unwrap();

    let p = Part {
        number: part_number,
        start,
        end,
    };
    return p;
}

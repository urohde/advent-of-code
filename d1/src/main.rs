use std::{fs::*, io::*, path::Path};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = read_lines(&args[1]).unwrap();

    let mut numbers: Vec<i32> = Vec::new();
    for line in lines {
        let line = line.unwrap();

        let mut first: Option<char> = None;
        let mut last: Option<char> = None;

        for char in line.chars() {
            if char.is_digit(10) {
                if first == None {
                    first = Some(char);
                } else {
                    last = Some(char);
                }
            }
        }

        let first_number = match first {
            Some(n) => n,
            None => panic!("No first digit found"),
        };

        let last_number = match last {
            Some(n) => n,
            None => first_number,
        };

        let number = format!("{}{}", first_number, last_number);

        println!("{} -> {}", line, number);
        numbers.push(number.parse::<i32>().unwrap());
    }

    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

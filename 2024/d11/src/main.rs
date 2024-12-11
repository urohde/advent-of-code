use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let loops = args[2].parse().expect("not a number");

    let reader = BufReader::new(File::open(args[1].clone()).expect("open failed"));

    let binding = reader
        .lines()
        .next()
        .expect("no lines")
        .expect("lines failed");

    let mut start: Vec<String> = binding.split(' ').map(|s| s.to_string()).collect();

    for _ in 0..loops {
        let mut next: Vec<String> = vec![];

        for number in start.iter() {
            // print!("number: {} ", number);
            if number == "0" {
                // print!("replace by 1");
                next.push("1".to_string());
            } else if number.len() % 2 == 0 {
                let half = number.len() / 2;
                let a = &number[0..half];
                let b = &number[half..];

                // print!("split a: {}, b: {}", a, b);

                next.push(a.to_string());
                let trim = b.trim_start_matches('0').to_string();
                if trim.len() == 0 {
                    next.push("0".to_string());
                } else {
                    next.push(trim);
                }
            } else {
                let n: i64 = number.parse().expect("not a number");
                let n: i64 = n * 2024;
                // print!("multiply: {}", n);
                next.push(n.to_string());
            }
            // print!("\n");
        }
        // print(&next);
        start = next;
    }
    println!("{}", start.len());
}

// fn print(s: &Vec<String>) {
//     for i in s.iter() {
//         print!("{} ", i);
//     }
//     print!("\n");
// }

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let reader = BufReader::new(File::open(args[1].clone()).expect("open failed"));

    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("regex syntax error");

    let mut sum: i32 = 0;
    for line in reader.lines() {
        let line = line.expect("lines failed");

        for (_, [a, b]) in re.captures_iter(&line).map(|c| c.extract()) {
            sum += a.parse::<i32>().expect("bad a") * b.parse::<i32>().expect("bad b");
        }
    }

    println!("{}", sum);
}

// fn main() {
//     let args: Vec<String> = std::env::args().collect();
//
//     let mut reader = BufReader::new(File::open(args[1].clone()).expect("open failed"));
//     let pattern: &[u8] = b"mul(";
//
//     let buffer_size = 32000;
//     let mut buf = vec![0; buffer_size + pattern.len() - 1];
//     let mut bytes_in_buffer = 0;
//
//     let mut sum = 0;
//
//     println!("{:?}", String::from_utf8_lossy(&buf));
//
//
//     loop {
//         let bytes_read = reader
//             .read(&mut buf[bytes_in_buffer..])
//             .expect("failed to read");
//
//         if bytes_read == 0 {
//             break;
//         }
//
//         bytes_in_buffer += bytes_read;
//
//         let mut i = 0;
//         while (i + pattern.len() - 1) <= bytes_in_buffer {
//             let pattern_end = i + pattern.len();
//
//             if i + pattern_end> bytes_read {
//                 break;
//             }
//
//             if buf[i..pattern_end] == *pattern {
//                 let closing = buf[pattern_end..pattern_end + 7]
//                     .to_vec()
//                     .iter()
//                     .position(|e| *e as char == ')');
//
//                 let closing = match closing {
//                     Some(c) => pattern_end + c,
//                     None => {
//                         i += 1;
//                         continue;
//                     }
//                 };
//
//                 let comma = buf[pattern_end..closing]
//                     .to_vec()
//                     .iter()
//                     .position(|e| *e as char == ',');
//
//                 let comma = match comma {
//                     Some(c) => pattern_end + c,
//                     None => {
//                         i += 1;
//                         continue;
//                     }
//                 };
//
//                 let a: i32 = match u8_array_to_i32(&buf[pattern_end..comma]) {
//                     Ok(v) => v,
//                     Err(..) => {
//                         i += 1;
//                         continue;
//                     }
//                 };
//
//                 let b: i32 = match u8_array_to_i32(&buf[comma + 1..closing]) {
//                     Ok(v) => v,
//                     Err(..) => {
//                         i += 1;
//                         continue;
//                     }
//                 };
//
//                 sum += a * b;
//             }
//             i += 1;
//         }
//
//         let overlap = pattern.len() - 1;
//         buf.copy_within(bytes_in_buffer - overlap..bytes_in_buffer, 0);
//         bytes_in_buffer = overlap;
//     }
//
//     println!("{}", sum);
// }
//
// fn u8_array_to_i32(bytes: &[u8]) -> Result<i32, std::num::ParseIntError> {
//     let s = String::from_utf8_lossy(bytes);
//     s.parse::<i32>()
// }

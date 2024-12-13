use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use pathfinding::prelude::dijkstra;
use regex::Regex;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i64, i64);

// impl Pos {
//     fn successors(&self) -> Vec<(Pos, usize)> {
//         let mut result = Vec::new();
//
//         let mut new_pos = self.clone();
//         new_pos.x += self.x_a;
//         new_pos.y += self.y_a;
//         result.push((new_pos, 3));
//
//         let mut new_pos = self.clone();
//         new_pos.x += self.x_b;
//         new_pos.y += self.y_b;
//         result.push((new_pos, 1));
//
//         return result;
//     }
// }

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let reader = BufReader::new(File::open(args[1].clone()).expect("open failed"));

    let mut lines = reader.lines();

    let pattern = r"(\d+), ..(\d+)";
    let re = Regex::new(pattern).expect("failed to create regex");

    let mut total_cost = 0;
    let mut loop_count = 0;

    let is_part_2 = args.len() > 2;

    loop {
        let line_a = lines.next().expect("no line").expect("line failed");
        let line_b = lines.next().expect("no line").expect("line failed");
        let line_target = lines.next().expect("no line").expect("line failed");

        let caps_a = re.captures(&line_a).expect("failed to match regex");
        let caps_b = re.captures(&line_b).expect("failed to match regex");
        let caps_target = re.captures(&line_target).expect("failed to match regex");

        let x_a: i64 = caps_a[1].parse().expect("failed to parse");
        let y_a: i64 = caps_a[2].parse().expect("failed to parse");

        let x_b: i64 = caps_b[1].parse().expect("failed to parse");
        let y_b: i64 = caps_b[2].parse().expect("failed to parse");

        let x: i64 = caps_target[1].parse::<i64>().expect("failed to parse")
            * if is_part_2 { 10000000000000 } else { 1 };
        let y: i64 = caps_target[2].parse::<i64>().expect("failed to parse")
            * if is_part_2 { 10000000000000 } else { 1 };

        let result = dijkstra(
            &Pos(0, 0),
            |p| {
                let mut result = Vec::new();

                if p.0 > x || p.1 > y {
                    return result;
                }
                let mut new_pos = p.clone();
                new_pos.0 += x_a;
                new_pos.1 += y_a;
                result.push((new_pos, 3));

                let mut new_pos = p.clone();
                new_pos.0 += x_b;
                new_pos.1 += y_b;
                result.push((new_pos, 1));

                return result;
            },
            |p| p.0 == x && p.1 == y,
        );

        print!("loop {}: ", loop_count);
        match result {
            Some((_, cost)) => {
                println!("cost: {}", cost);
                total_cost += cost;
            }
            None => {
                println!("no path found");
            }
        }

        loop_count += 1;
        match lines.next() {
            Some(_) => (),
            None => break,
        }
    }

    println!("total cost: {}", total_cost);
}

// fn buttons(a: Button, b: Button, x: i32, y: i32) -> Option<(i32, i32)> {
//     let mut a_is_better = false;
//
//     if a.x > 3 * b.x && a.y > 3 * b.y {
//         println!("a is better");
//         a_is_better = true;
//     }
//
//     let single_solve_a = check_single_button_solve(&a, &x, &y);
//     let single_solve_b = check_single_button_solve(&b, &x, &y);
//
//     if a_is_better {
//         if let Some(n) = single_solve_a {
//             return Some((n, 0));
//         }
//     }
//
//     if let Some(n) = single_solve_b {
//         return Some((0, n));
//     }
//
//     if a_is_better {
//         match brute_force(&b, &a, &x, &y) {
//             Some(r) => {
//                 return Some((r.1, r.0));
//             }
//             None => (),
//         }
//     }
//
//     return brute_force(&a, &b, &x, &y);
// }
//
// fn check_single_button_solve(button: &Button, x: &i32, y: &i32) -> Option<i32> {
//     if x / button.x == y / button.y && button.x % 2 == 0 {
//         return Some(
//             (x / button.x)
//                 .try_into()
//                 .expect("failed to convert i32 to usize"),
//         );
//     }
//     return None;
// }
//
// fn brute_force(a: &Button, b: &Button, x: &i32, y: &i32) -> Option<(i32, i32)> {
//     let mut a_presses = 0;
//     let mut b_presses = 0;
//
//     let mut x_1 = a_presses * a.x + b_presses * b.x;
//     let mut y_1 = a_presses * a.y + b_presses * b.y;
//
//     let a_increments_x = a.x > b.x;
//     let a_increments_y = a.y > b.y;
//
//     while *x != x_1 && *y != y_1 {
//         if a_presses < 0 || b_presses < 0 {
//             return None;
//         }
//
//         if x_1 < *x {
//             if a_increments_x {
//                 a_presses += 1;
//             } else {
//                 b_presses += 1;
//             }
//         }
//
//         if y_1 < *y {
//             if a_increments_y {
//                 a_presses += 1;
//             } else {
//                 b_presses += 1;
//             }
//         }
//
//         if x_1 > *x {
//             if a_increments_x {
//                 a_presses -= 1;
//             } else {
//                 b_presses -= 1;
//             }
//         }
//
//         if y_1 > *y {
//             if a_increments_y {
//                 a_presses -= 1;
//             } else {
//                 b_presses -= 1;
//             }
//         }
//
//         x_1 = a_presses * a.x + b_presses * b.x;
//         y_1 = a_presses * a.y + b_presses * b.y;
//     }
//
//     return Some((a_presses, b_presses));
// }
//
// fn calculate_cost(a: &i32, b: &i32) -> i32 {
//     return 3 * a + b;
// }

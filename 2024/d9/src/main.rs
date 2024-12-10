use std::{
    cmp::min,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let reader = BufReader::new(File::open(args[1].clone()).expect("open failed"));

    let mut line: Vec<char> = reader.lines().next().unwrap().unwrap().chars().collect();
    let mut debug_line = vec![];

    let mut block = 0;
    let mut block_end = line.len() / 2;

    let mut start_ptr = 0;
    let mut end_ptr = line.len() - 1;
    let mut real_ptr = 0;

    let mut sum = 0;

    // println!(
    //     "init start: {}, end: {}, block: {}, block end: {}",
    //     start_ptr, end_ptr, block, block_end
    // );

    while start_ptr <= end_ptr {
        let num = line[start_ptr].to_digit(10).expect("not a digit");
        match start_ptr % 2 {
            0 => {
                // println!("data");
                for _ in 0..num {
                    sum += real_ptr * block;
                    real_ptr += 1;
                    debug_line.push(block);
                }
                start_ptr += 1;
                block += 1;
            }
            _ => {
                // print!("hole ");
                if end_ptr % 2 != 0 {
                    end_ptr -= 1;
                    // print!("end is hole");
                    // print!("\n");
                    continue;
                }
                let last_block_num = line[end_ptr].to_digit(10).expect("not a digit");
                if last_block_num == 0 {
                    end_ptr -= 1;
                    // print!("end is zero");
                    // print!("\n");
                    continue;
                }
                // print!("num: {} ", num);
                // print!("last num: {} ", last_block_num);
                // print!("block end: {} ", block_end);

                let smallest = min(num, last_block_num);
                for _ in 0..smallest {
                    // print!("loop ");
                    sum += real_ptr * block_end;
                    real_ptr += 1;
                    debug_line.push(block_end);
                }

                if num == last_block_num {
                    // print!("c");
                    start_ptr += 1;
                    end_ptr -= 1;
                    block_end -= 1;
                } else if smallest == num {
                    // print!("a");
                    line[end_ptr] = (last_block_num - num)
                        .to_string()
                        .chars()
                        .next()
                        .expect("no digits");
                    start_ptr += 1;
                } else if smallest == last_block_num {
                    // print!("b");
                    line[start_ptr] = (num - last_block_num)
                        .to_string()
                        .chars()
                        .next()
                        .expect("no digits");
                    end_ptr -= 1;
                    block_end -= 1;
                }

                // print!("\n");
            }
        }
    }

    // for n in debug_line.iter() {
    //     print!("{n}");
    // }
    // print!("\n");

    println!("{}", sum);
}

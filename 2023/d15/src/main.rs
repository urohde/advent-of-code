use std::{
    fs::*,
    io::{BufRead, BufReader, Result},
};

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut reader = BufReader::new(File::open(&args[1])?);

    let mut hash_sum: i32 = 0;
    loop {
        let mut buf = Vec::new();
        let n = reader.read_until(b',', &mut buf).unwrap();
        println!("{:?}", buf);
        let mut token_sum: i32 = 0;
        for b in buf {
            match b {
                b'\n' => break,
                b',' => continue,
                _ => token_sum = hash(token_sum, b),
            }
        }
        println!("Token sum: {}", token_sum);
        hash_sum += token_sum;
        if n == 0 {
            break;
        }
    }
    println!("{}", hash_sum);
    Ok(())
}

fn hash(mut hash: i32, b: u8) -> i32 {
    print!("{}:", b);
    print!(" {} -> ", hash);
    hash += b as i32;
    print!("{} ->", hash);
    hash *= 17;
    print!("{} ->", hash);
    hash %= 256;
    print!("{}\n", hash);
    return hash;
}

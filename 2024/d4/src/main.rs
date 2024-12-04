use std::{
    fs::File,
    io::{BufRead, BufReader},
    usize,
};

#[derive(Debug, Clone, Copy)]
struct Target {
    index: usize,
    next: usize,
    word_index: usize,
    word_index_direction: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let reader = BufReader::new(File::open(args[1].clone()).expect("open failed"));
    let word = args[2].as_bytes();

    let mut count = 0;
    let mut line_length;

    let mut targets: Vec<Target> = vec![];

    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        line_length = line.len() - 1;
        println!("{}: {}", i, line);
        for (j, char) in line.chars().enumerate() {
            let here = i * line_length + j;
            println!("{}", here);
            let target_index = targets.iter().position(|t| t.index == here);
            let target = match target_index {
                Some(i) => Some(targets[i]),
                None => None,
            };
            'match_target: {
                match target {
                    Some(ref t) => {
                        if t.index != here {
                            // no hit, keep looking
                            break 'match_target;
                        }

                        if word[t.word_index] as char != char {
                            println!("wrong char");
                            // not the correct char, move on
                            // target = targets.pop();
                            break 'match_target;
                        }

                        println!("Found match {:?}", t);

                        if t.word_index == word.len() - 1 {
                            println!("increasing count");
                            // hit last char, increase count
                            count += 1;
                            // target = targets.pop();
                            break 'match_target;
                        }

                        let mut next_word_index = t.word_index + 1;
                        if !t.word_index_direction {
                            next_word_index -= 2;
                        }

                        let new_t = Target {
                            index: t.index + t.next,
                            next: t.next,
                            word_index: next_word_index,
                            word_index_direction: t.word_index_direction,
                        };

                        println!("Next {:?}", new_t);
                        if t.next == 1 {
                            targets.insert(0, new_t);
                        } else {
                            targets.push(new_t);
                        }

                        // target = targets.pop();
                    }
                    None => {
                        // target = targets.pop();
                    }
                }
            }

            if char == word[0] as char {
                // if let Some(t) = target {
                //     targets.insert(0, t);
                // }
                let t = create_targets(here, 1, true, line_length);
                // target = Some(t.0);
                targets.push(t.0);
                targets.push(t.1);
                targets.push(t.2);
                targets.push(t.3);
            }

            if char == word[word.len() - 1] as char {
                // if let Some(t) = target {
                //     targets.insert(0, t);
                // }
                let t = create_targets(here, word.len() - 1, false, line_length);
                // target = Some(t.0);
                targets.push(t.0);
                targets.push(t.1);
                targets.push(t.2);
                targets.push(t.3);
            }

            // if target.is_none() {
            //     target = targets.pop();
            // }
        }
    }

    println!("{}", count)
}

fn create_targets(
    here: usize,
    word_index: usize,
    word_index_direction: bool,
    line_length: usize,
) -> (Target, Target, Target, Target) {
    return (
        Target {
            index: here + 1,
            next: 1,
            word_index,
            word_index_direction,
        },
        Target {
            index: here + line_length,
            next: line_length,
            word_index,
            word_index_direction,
        },
        Target {
            index: here + line_length,
            next: line_length + 1,
            word_index,
            word_index_direction,
        },
        Target {
            index: here + line_length - 1,
            next: line_length - 1,
            word_index,
            word_index_direction,
        },
    );
}

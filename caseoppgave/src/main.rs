use chrono::Datelike;
use chrono::{prelude::DateTime, Utc};
use rustc_hash::FxHashMap as HashMap;
use std::fs::read_to_string;
use std::time::{Duration, Instant, UNIX_EPOCH};
use std::{fs::*, io::*};

#[derive(PartialEq, Eq, Hash)]
struct MeterDirectionMonth<'a> {
    meter_id: &'a str,
    direction: &'a str,
    year: i32,
    month: u32,
}

#[derive(PartialEq, Eq, Hash)]
struct GroupDirectionMonth<'a> {
    group: &'a str,
    year: i32,
    month: u32,
    direction: &'a str,
}

#[derive(PartialEq, Eq, Hash)]
struct GroupDirection<'a> {
    group: &'a str,
    direction: &'a str,
}

enum Error<'a> {
    UnknownDirection(&'a str, usize),
    NegativeValue(&'a str, usize),
    FutureTimestamp(&'a str, usize),
}

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = std::env::args().collect();

    let lines = read_to_string(&args[1]).unwrap();
    let lines = lines.lines();

    let now = Utc::now().timestamp();

    let mut meterid_direction_month: HashMap<MeterDirectionMonth, f64> = HashMap::default();
    let mut group_month_direction: HashMap<GroupDirectionMonth, f64> = HashMap::default();
    let mut group_direction: HashMap<GroupDirection, f64> = HashMap::default();
    let mut errors: Vec<Error> = Vec::new();

    let mut i = 0;
    for line in lines {
        let mut parts = line.split(',');

        let meter_id = parts.next().unwrap();
        let timestamp = parts.next().unwrap().parse::<i64>().unwrap();
        let value = parts.next().unwrap().parse::<f64>().unwrap();
        let group = parts.next().unwrap();
        let direction = parts.next().unwrap().trim();

        let d = UNIX_EPOCH + Duration::from_secs(timestamp as u64);
        let date_time = DateTime::<Utc>::from(d);
        let year = date_time.year();
        let month = date_time.month();

        if direction != "UP" && direction != "DOWN" {
            errors.push(Error::UnknownDirection(line, i));
            continue;
        }

        if value < 0.0 {
            errors.push(Error::NegativeValue(line, i));
            continue;
        }

        if timestamp > now {
            errors.push(Error::FutureTimestamp(line, i));
            continue;
        }

        meterid_direction_month
            .entry(MeterDirectionMonth {
                meter_id,
                direction,
                year,
                month,
            })
            .and_modify(|v| *v += value)
            .or_insert(value);

        group_month_direction
            .entry(GroupDirectionMonth {
                group,
                year,
                month,
                direction,
            })
            .and_modify(|v| *v += value)
            .or_insert(value);

        group_direction
            .entry(GroupDirection { group, direction })
            .and_modify(|v| *v += value)
            .or_insert(value);

        i += 1;
    }

    write_mdm_to_file("output1.csv", meterid_direction_month);
    write_gdm_to_file("output2.csv", group_month_direction);
    write_gd_to_file("output3.csv", group_direction);
    write_errors_to_file("errors.csv", errors);

    print!("Time elapsed: {:?}\n", start_time.elapsed());
}

fn create_or_open(file_name: &str) -> BufWriter<File> {
    BufWriter::new(match File::create(file_name) {
        Ok(file) => file,
        Err(_) => panic!("Failed to create file {}", file_name),
    })
}

fn write_mdm_to_file(filename: &str, map: HashMap<MeterDirectionMonth, f64>) {
    let mut file = create_or_open(filename);
    print!("Writing to file: {}\n\t{} entries\n", filename, map.len());
    map.iter().for_each(|(k, v)| {
        write!(
            file,
            "{},{},{},{},{:.3}\n",
            k.meter_id,
            k.direction,
            k.year,
            get_month(k.month),
            v
        )
        .unwrap();
    });
    match file.flush() {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Failed to write to file {}", filename)
        }
    }
}

fn write_gdm_to_file(filename: &str, map: HashMap<GroupDirectionMonth, f64>) {
    let mut file = create_or_open(filename);
    print!("Writing to file: {}\n\t{} entries\n", filename, map.len());
    map.iter().for_each(|(k, v)| {
        write!(
            file,
            "{},{},{},{},{:.3}\n",
            k.group,
            k.year,
            get_month(k.month),
            k.direction,
            v
        )
        .unwrap();
    });
    match file.flush() {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Failed to write to file {}", filename)
        }
    }
}

fn write_gd_to_file(filename: &str, map: HashMap<GroupDirection, f64>) {
    let mut file = create_or_open(filename);
    print!("Writing to file: {}\n\t{} entries\n", filename, map.len());
    map.iter().for_each(|(k, v)| {
        write!(file, "{},{},{:.3}\n", k.group, k.direction, v).unwrap();
    });
    match file.flush() {
        Ok(_) => {}
        Err(_) => {
            eprintln!("Failed to write to file {}", filename)
        }
    }
}

fn write_errors_to_file(filename: &str, errors: Vec<Error<'_>>) {
    let mut error_writer = create_or_open(filename);
    print!("Writing to file: {}\n\t{} entries\n", filename, errors.len());
    errors.iter().for_each(|e| {
        match e {
            Error::UnknownDirection(line, line_number) => {
                write!(error_writer, "{},{}\n", line_number, line).unwrap()
            }
            Error::NegativeValue(line, line_number) => {
                write!(error_writer, "{},{}\n", line_number, line).unwrap()
            }
            Error::FutureTimestamp(line, line_number) => {
                write!(error_writer, "{},{}\n", line_number, line).unwrap()
            }
        };
    });
}


fn get_month(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

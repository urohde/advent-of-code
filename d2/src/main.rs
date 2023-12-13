use core::fmt;
use std::{
    cmp::max,
    fs::*,
    io::*,
    path::Path,
};

struct Game {
    name: String,
    red: i32,
    blue: i32,
    green: i32,
    power: i32,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Game {} [{}] \n\t{} red\n\t{} blue\n\t{} green",
            self.name, self.power, self.red, self.blue, self.green
        )
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lines = read_lines(&args[1]).unwrap();

    let red = args[2].parse::<i32>().unwrap();
    let green = args[3].parse::<i32>().unwrap();
    let blue = args[4].parse::<i32>().unwrap();

    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        let line = line.unwrap();

        let game = parse_game(&line);
        println!("{}", game);
        games.push(game);
    }

    let possible_games: Vec<&Game> = games
        .iter()
        .filter(|&game| game.red <= red && game.blue <= blue && game.green <= green)
        .collect();

    println!("Possible games: {}", possible_games.iter().map(|game| game.name.clone()).collect::<Vec<String>>().join(", "));
    let possible_game_sum = possible_games.iter().map(|game| game.name.parse::<i32>().unwrap()).sum::<i32>();
    println!("Possible games sum: {}", possible_game_sum);

    let power_sum = games.iter().map(|game| game.power).sum::<i32>();
    println!("Possible games power sum: {}", power_sum);
}

fn parse_game(line: &String) -> Game {
    let name_game: Vec<&str> = line.split(":").collect();
    let hands: Vec<&str> = name_game[1].split(";").collect();

    let mut game = Game {
        name: name_game[0].split(" ").collect::<Vec<&str>>()[1].to_string(),
        red: 0,
        blue: 0,
        green: 0,
        power: 0,
    };

    for hand in hands {
        let cubes: Vec<&str> = hand.split(",").collect();
        for cube in cubes {
            let number_color: Vec<&str> = cube.trim().split(" ").collect();
            let number: i32 = number_color[0].parse::<i32>().unwrap();
            let color: &str = number_color[1];
            match color {
                "red" => game.red = max(game.red, number),
                "blue" => game.blue = max(game.blue, number),
                "green" => game.green = max(game.green, number),
                _ => panic!("Unknown color: {}", color),
            }
        }
    }

    game.power = game.red * game.blue * game.green;

    return game;
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

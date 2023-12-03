#[path = "./utils.rs"]
mod utils;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

pub fn sum_possible_game_ids() -> u32 {
    let file_path = "./src/input/day_two.txt";

    let mut possible_game_ids: Vec<u32> = Vec::new();

    for line in utils::lines_from_file(file_path) {
        let game = parse_game(line);
        if game.possible {
            possible_game_ids.push(game.id);
        }
    }

    let mut sum: u32 = 0;

    for id in possible_game_ids {
        sum += id;
    }

    return sum;
}

pub fn sum_power_of_cubes() -> u32 {
    let file_path = "./src/input/day_two.txt";

    let mut games: Vec<Game> = Vec::new();

    for line in utils::lines_from_file(file_path) {
        games.push(parse_game(line));
    }

    let mut sum: u32 = 0;

    for g in games {
        sum += g.red_max * g.green_max * g.blue_max;
    }

    return sum;
}

struct GameReveal {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    possible: bool,
    red_max: u32,
    green_max: u32,
    blue_max: u32,
    reveals: Vec<GameReveal>,
}

// Game 1: 1 green, 2 blue; 13 red, 2 blue, 3 green; 4 green, 14 red
fn parse_game(line: String) -> Game {
    let game_split: Vec<&str> = line.split(":").collect();
    let game_id = game_split.first().unwrap().split(" ").last().unwrap();

    let mut game = Game {
        id: game_id.parse::<u32>().unwrap(),
        possible: true,
        red_max: 0,
        green_max: 0,
        blue_max: 0,
        reveals: Vec::new(),
    };

    let all_reveals: Vec<&str> = game_split.last().unwrap().split(";").collect();

    for a in all_reveals {
        let block_reveals = a.trim().split(",");

        for b in block_reveals {
            let mut game_reveal = GameReveal {
                red: 0,
                green: 0,
                blue: 0,
            };

            let stats: Vec<&str> = b.trim().split(" ").collect();

            let count = stats.first().unwrap().parse::<u32>().unwrap();
            let color = stats.last().unwrap();

            if *color == "red" {
                if count > game.red_max {
                    game.red_max = count;
                }
                game_reveal.red = count;
            }

            if *color == "green" {
                if count > game.green_max {
                    game.green_max = count;
                }
                game_reveal.green = count;
            }

            if *color == "blue" {
                if count > game.blue_max {
                    game.blue_max = count;
                }
                game_reveal.blue = count;
            }

            if game_reveal.red > MAX_RED
                || game_reveal.green > MAX_GREEN
                || game_reveal.blue > MAX_BLUE
            {
                game.possible = false;
            }

            game.reveals.push(game_reveal);
        }
    }

    return game;
}

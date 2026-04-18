use clap::Parser;
use serde_derive::{Deserialize, Serialize};

use std::{collections::HashMap, process::Command};

#[derive(Serialize, Deserialize)]
struct Config {
    games: HashMap<String, String>,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            games: HashMap::new(),
        }
    }
}

#[derive(Parser)]
struct Cli {
    ///Add a game to your list, requires both game name and id
    #[arg(short, long, default_value_t = false)]
    add: bool,
    ///Remove a game from your list, requires the game name
    #[arg(short, long, default_value_t = false)]
    remove: bool,
    ///Display a list of your games
    #[arg(short, long, default_value_t = false)]
    list: bool,

    ///Game name, Example: "Space Game"
    game: Option<String>,
    ///Game Id, Example: "123456"
    id: Option<String>,
}

fn main() {
    let cfg: Config = confy::load("steamruster", None).unwrap();

    let args = Cli::parse();

    if args.add {
        addgame(cfg, args.game, args.id);
    } else if args.remove {
        delgame(cfg, args.game);
    } else if args.list {
        listgames(cfg.games);
    } else {
        loadgame(cfg.games, args.game);
    }
}

fn loadgame(games: HashMap<String, String>, game_option: Option<String>) {
    match game_option {
        Some(key) => {
            println!("Searching for: {}", key);
            if games.contains_key(&key) {
                match games.get(&key) {
                    Some(game) => {
                        println!("Found Game with id: {}", game);
                        let mut game_command = "steam://rungameid/".to_string();
                        game_command.push_str(game);
                        let _command = Command::new("steam")
                            .arg(game_command)
                            .output()
                            .expect("Failed to run game");
                    }
                    None => eprintln!("Failed to run game"),
                }
            } else {
                eprintln!("Could not find game")
            }
        }
        None => {
            eprintln!("Please enter the name of a game");
        }
    }
}

fn addgame(mut cfg: Config, name_option: Option<String>, id_option: Option<String>) {
    match name_option {
        Some(name) => match id_option {
            Some(id) => {
                println!("Adding {} with id: {}", name, id);

                if cfg.games.contains_key(&name) {
                    eprintln!("Game already exists");
                    return;
                }
                cfg.games.insert(name, id);
                confy::store("steamruster", None, cfg).unwrap();
                println!("Added game");
            }
            None => eprintln!("Please enter the id for your game"),
        },
        None => eprintln!("Please enter the name of a game"),
    }
}

fn delgame(mut cfg: Config, name_option: Option<String>) {
    match name_option {
        Some(name) => {
            println!("Deleting {}", name);

            if !cfg.games.contains_key(&name) {
                eprintln!("Could not find game");
                return;
            }
            cfg.games.remove(&name);
            confy::store("steamruster", None, cfg).unwrap();
            println!("{} Deleted", name);
        }
        None => eprintln!("Please enter the name of a game"),
    }
}

fn listgames(games: HashMap<String, String>) {
    println!("Name: ID");
    for (key, value) in &games {
        println!("{}: {}", key, value);
    }
}

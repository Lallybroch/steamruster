use clap::Parser;
use serde_derive::{Deserialize, Serialize};

use std::{collections::HashMap, process::Command};

//Stardew command "steam steam://rungameid/413150"
//

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
    #[arg(short, long, default_value_t = false)]
    add: bool,
    #[arg(short, long, default_value_t = false)]
    remove: bool,
    #[arg(short, long, default_value_t = String::new())]
    game: String,
    #[arg(short, long, default_value_t = String::new())]
    id: String,
    #[arg(short, long, default_value_t = false)]
    list: bool,
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

    /*
    let _command = Command::new("steam")
        .arg("steam://rungameid/413150")
        .output()
        .expect("Failed to run game");
    */
}

fn loadgame(games: HashMap<String, String>, key: String) {
    if key == String::new() {
        eprintln!("Please enter the name of a game\nExample: --game 'awesomegame'");
        return;
    }
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

fn addgame(mut cfg: Config, name: String, id: String) {
    println!("Adding {} with id: {}", name, id);
    if id == String::new() {
        eprintln!("Please enter the id for the game\nExample: --id '123456'");
        return;
    }
    if name == String::new() {
        eprintln!("Please enter the name of a game\nExample: --game 'awesomegame'");
        return;
    }
    if cfg.games.contains_key(&name) {
        eprintln!("Game already exists");
        return;
    }
    cfg.games.insert(name, id);
    confy::store("steamruster", None, cfg).unwrap();
    println!("Added game");
}

fn delgame(mut cfg: Config, name: String) {
    println!("Deleting {}", name);
    if name == String::new() {
        eprintln!("Please enter the name of a game\nExample: --game 'awesomegame'");
        return;
    }
    if !cfg.games.contains_key(&name) {
        eprintln!("Could not find game");
        return;
    }
    cfg.games.remove(&name);
    confy::store("steamruster", None, cfg).unwrap();
    println!("{} Deleted", name);
}

fn listgames(games: HashMap<String, String>) {
    println!("Name: ID");
    for (key, value) in &games {
        println!("{}: {}", key, value);
    }
}

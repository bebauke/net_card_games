mod deck;
mod hand;
mod player; // Import the entire `player` module
mod spiele; // Import the entire `spiele` module

use std::io;
use crate::spiele::{hosn_obi::HosnObiGame, schnapsen::SchnapsenGame, skat::SkatGame, Game};
use crate::player::{PlayerType, PlayerFactory, Player}; // Import the Player module

/// Enum for game selection
enum GameSelection {
    HosnObi,
    Schnapsen,
    Skat,
}

fn main() {
    loop {
        // Game selection
        let game_selection = get_game_selection();

        if let Some(game_selection) = game_selection {
            // Ask for the number of players
            let num_players = get_number_of_players();

            // Ask for player types and initialize players
            let player_types = get_player_types(num_players);
            let players: Vec<Box<dyn Player>> = initialize_players(player_types);

            // Initialize the game
            let mut game: Box<dyn Game> = match game_selection {
                GameSelection::HosnObi => Box::new(HosnObiGame::new(players)),
                GameSelection::Schnapsen => Box::new(SchnapsenGame::new(players)),
                GameSelection::Skat => Box::new(SkatGame::new(players)),
            };

            // Start and manage the game
            game.start_game();

            // Output game status
            println!("Game status: {}", game.status());

            // Option to reset or quit the game
            println!("Do you want to reset the game or quit? (r = reset, q = quit)");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            match input.trim() {
                "r" => {
                    game.reset();
                    println!("The game has been reset.");
                }
                "q" => {
                    println!("Game ended.");
                    break;
                }
                _ => println!("Invalid input. Game continues."),
            }
        } else {
            println!("Game ended.");
            break;
        }
    }
}

/// Function to get the game selection
fn get_game_selection() -> Option<GameSelection> {
    loop {
        println!("Choose a game:");
        println!("1: Hosn Obi");
        println!("2: Schnapsen");
        println!("3: Skat");
        println!("q: Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => return Some(GameSelection::HosnObi),
            "2" => return Some(GameSelection::Schnapsen),
            "3" => return Some(GameSelection::Skat),
            "q" => return None,
            _ => println!("Invalid selection, please try again."),
        }
    }
}

/// Function to get the number of players
fn get_number_of_players() -> usize {
    println!("Enter the number of players:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<usize>().unwrap_or(3) // Default: 3 players
}

/// Function to ask for the type of each player
fn get_player_types(num_players: usize) -> Vec<PlayerType> {
    let mut player_types = Vec::new();

    for i in 0..num_players {
        loop {
            println!("Select the type for player {}:", i + 1);
            println!("1: Local Player");
            println!("2: AI Player");
            println!("3: Network Player");

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim() {
                "1" => {
                    player_types.push(PlayerType::Local);
                    break;
                }
                "2" => {
                    player_types.push(PlayerType::AI);
                    break;
                }
                "3" => {
                    player_types.push(PlayerType::Network);
                    break;
                }
                _ => println!("Invalid selection, please try again."),
            }
        }
    }

    player_types
}

/// Function to initialize players based on their types
fn initialize_players(player_types: Vec<PlayerType>) -> Vec<Box<dyn Player>> {
    player_types.into_iter().map(|player_type| {
        PlayerFactory::create_player(player_type)
    }).collect()
}

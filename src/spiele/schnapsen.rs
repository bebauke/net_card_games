// schnapsen.rs

use crate::player::Player; // Import the Player trait
use crate::Game;

pub struct SchnapsenGame {
    players: Vec<Box<dyn Player>>,
}

impl SchnapsenGame {
    pub fn new(players: Vec<Box<dyn Player>>) -> Self {
        Self { players }
    }
}

impl Game for SchnapsenGame {
    fn start_game(&mut self) {
        println!("Schnapsen Game started with {} players!", self.players.len());
    }

    fn status(&self) -> String {
        "Game in progress".to_string()
    }

    fn reset(&mut self) {
        println!("Schnapsen Game has been reset.");
    }
}

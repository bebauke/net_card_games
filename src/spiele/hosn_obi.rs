// hosn_obi.rs
use crate::player::Player;
use crate::Game; // Import the Game trait

pub struct HosnObiGame {
    players: Vec<Box<dyn Player>>,
}

impl HosnObiGame {
    // Constructor for HosnObiGame
    pub fn new(players: Vec<Box<dyn Player>>) -> Self {
        Self { players }
    }
}

impl Game for HosnObiGame {
    fn start_game(&mut self) {
        println!("Hosn Obi Game started with {} players!", self.players.len());
    }

    fn status(&self) -> String {
        "Game in progress".to_string()
    }

    fn reset(&mut self) {
        println!("Hosn Obi Game has been reset.");
    }
}

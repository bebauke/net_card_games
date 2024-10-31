use crate::player::Player;
use crate::Game;
use crate::deck::{Deck,Card,DeckType};

pub struct SkatGame {
    players: Vec<Box<dyn Player>>,
}

impl SkatGame {
    pub fn new(players: Vec<Box<dyn Player>>) -> Self {
        Self { players }
    }

    // Frage alle Spieler nach ihrem Namen
    
}

impl Game for SkatGame{
    fn start_game(&mut self) {
        println!("Skat Game started with {} players!", self.players.len());
    }

    fn status(&self) -> String {
        "Game in progress".to_string()
    }

    fn reset(&mut self) {
        println!("Skat Game has been reset.");
    }
}
pub mod com;
pub mod loc;
pub mod net;

use crate::deck::{Card, Deck};
use crate::hand::CardState;
use crate::player::com::AIPlayer;
use crate::player::loc::LocalPlayer;
use crate::player::net::NetworkPlayer;


/// Trait zur Definition eines Spielers
pub trait Player {
    fn get_player_name(&self) -> String;
    fn draw_card(&mut self, deck: &mut Deck);
    fn play_card(&mut self) -> Option<Card>;
    fn show_hand(&self);
    fn make_move(&self) -> usize;
    fn is_hand_empty(&self) -> bool;
    fn update_card_state(&mut self, index: usize, state: CardState);
    fn validate_move(&self, card: &Card) -> bool;
    fn message(&self, string:String);
}

/// Enum zur Definition der Spielertypen
pub enum PlayerType {
    Local,
    AI,
    Network,
}

/// Struktur zur Erstellung von Spielern
pub struct PlayerFactory;

impl PlayerFactory {
    /// Methode zur Erstellung eines Spielers basierend auf dem `PlayerType`
    pub fn create_player(player_type: PlayerType) -> Box<dyn Player> {
        match player_type {
            PlayerType::Local => Box::new(LocalPlayer::new()), // Beispiel-Implementierung
            PlayerType::AI => Box::new(AIPlayer::new()),       // Beispiel-Implementierung
            PlayerType::Network => Box::new(NetworkPlayer::new()), // Beispiel-Implementierung
        }
    }
}
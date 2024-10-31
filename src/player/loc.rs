use crate::deck::{Card, Deck};
use crate::hand::CardState;
use crate::Player; 

pub struct LocalPlayer {
    name: String,
    // Weitere Attribute wie Handkarten
}

impl LocalPlayer {
    pub fn new() -> Self {
        let name = String::from("LocalPlayer");
        Self { name }
    }
}

impl Player for LocalPlayer {
    fn get_player_name(&self) -> String {
        self.name.clone()
    }
    
    fn draw_card(&mut self, deck: &mut Deck) {
        // Implementiere das Ziehen einer Karte
    }

    fn play_card(&mut self) -> Option<Card> {
        // Implementiere das Spielen einer Karte
        None
    }

    fn show_hand(&self) {
        // Implementiere das Anzeigen der Handkarten
    }

    fn make_move(&self) -> usize {
        // Implementiere die Logik für den Zug
        0
    }

    fn is_hand_empty(&self) -> bool {
        // Überprüfe, ob die Hand leer ist
        true
    }

    fn update_card_state(&mut self, index: usize, state: CardState) {
        // Aktualisiere den Zustand einer Karte
    }

    fn validate_move(&self, card: &Card) -> bool {
        // Validiere den Zug
        true
    }

    fn message(&self, string:String) {
        
    }
}

// Ähnliche Implementierungen für `AIPlayer` und `NetworkPlayer` folgen...

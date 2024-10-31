use crate::deck::{Card, Deck};
use crate::hand::CardState;
use crate::Player;

pub struct AIPlayer {
    name: String,
    hand: Vec<(Card, CardState)>, // Karten auf der Hand mit ihrem Zustand
}

impl AIPlayer {
    pub fn new() -> Self {
        Self {
            name: String::from("COM-Player"),
            hand: Vec::new(),
        }
    }
}

impl Player for AIPlayer {
    fn get_player_name(&self) -> String {
        self.name.clone()
    }

    fn draw_card(&mut self, deck: &mut Deck) {
        if let Some(card) = deck.pull() {
            self.hand.push((card, CardState::Closed));
        }
    }

    fn play_card(&mut self) -> Option<Card> {
        if let Some((card, _)) = self.hand.pop() {
            Some(card) // Die KI spielt die oberste Karte
        } else {
            None
        }
    }

    fn show_hand(&self) {
        println!("{}'s Hand:", self.name);
        for (card, state) in &self.hand {
            match state {
                CardState::Open => print!("Aufgedeckt: "),
                CardState::Closed => print!("Zugedeckt: "),
            }
            println!("Karte: {:?}", card); // Zum Debuggen
        }
    }

    fn make_move(&self) -> usize {
        // Einfaches KI-Verhalten: immer die erste Karte spielen
        0
    }

    fn is_hand_empty(&self) -> bool {
        self.hand.is_empty()
    }

    fn update_card_state(&mut self, index: usize, state: CardState) {
        if let Some(card) = self.hand.get_mut(index) {
            card.1 = state;
        }
    }

    fn validate_move(&self, _card: &Card) -> bool {
        // Einfache Validierung: Die KI akzeptiert immer ihre eigenen Züge
        true
    }
    
    fn message(&self, _string:String) {
        todo!()
    }
}

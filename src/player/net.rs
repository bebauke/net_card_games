use crate::deck::{Card, Deck};
use crate::hand::CardState;
use crate::Player;

pub struct NetworkPlayer {
    name: String,
    hand: Vec<(Card, CardState)>, // Karten auf der Hand mit ihrem Zustand
}

impl NetworkPlayer {
    pub fn new() -> Self {
        Self {
            name: String::from("NetworkPlayer"),
            hand: Vec::new(),
        }
    }
}

impl Player for NetworkPlayer {
    fn get_player_name(&self) -> String {
        self.name.clone()
    }

    fn draw_card(&mut self, deck: &mut Deck) {
        // Simuliere das Ziehen einer Karte aus dem Netzwerk
        if let Some(card) = deck.pull() {
            self.hand.push((card, CardState::Closed));
        }
    }

    fn play_card(&mut self) -> Option<Card> {
        // Simuliere das Spielen einer Karte über das Netzwerk
        if let Some((card, _)) = self.hand.pop() {
            Some(card)
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
            println!("Karte: {:?}", card); 
        }
    }

    fn make_move(&self) -> usize {
        // Platzhalter: Empfange einen Zug über das Netzwerk
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
        // Platzhalter: Validierung über das Netzwerk
        true
    }
    
    fn message(&self, _string:String) {
        todo!()
    }
}

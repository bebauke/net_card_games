use crate::deck::{self, Card, Deck}; // Importiere `Card` und `Deck` aus `deck.rs`
use std::rc::Rc;
use std::cell::RefCell;

/// Enum zur Repräsentation des Zustands einer Karte
#[derive(Copy, Clone)]
pub enum CardState {
    Open,   // Karte ist aufgedeckt
    Closed, // Karte ist zugedeckt
}

/// Struktur zur Repräsentation einer Hand oder der Karten auf dem Tisch
pub struct Hand {
    hand_cards: Vec<(Card, CardState)>,  // Karten auf der Hand mit ihrem Zustand
    table_cards: Vec<(Card, CardState)>, // Karten auf dem Tisch mit ihrem Zustand
    deck: Rc<RefCell<Deck>>,             // Gemeinsames Deck, das von allen Spielern geteilt wird
}

impl Hand {
    /// Konstruktor für eine neue Hand
    pub fn new(deck: Rc<RefCell<Deck>>) -> Self {
        Self {
            hand_cards: Vec::new(),
            table_cards: Vec::new(),
            deck,
        }
    }

    /// Methode, um eine Karte zur Hand hinzuzufügen
    pub fn add_card_to_hand(&mut self, card: Card, state: CardState) {
        self.hand_cards.push((card, state));
    }

    /// Methode, um eine Karte auf den Tisch zu legen
    pub fn add_card_to_table(&mut self, card: Card, state: CardState) {
        self.table_cards.push((card, state));
    }

    /// Methode, um die Karten auf der Hand auszugeben
    pub fn print_hand(&self) {
        println!("Karten auf der Hand:");
        for (card, state) in &self.hand_cards {
            match state {
                CardState::Open => print!("Aufgedeckt: "),
                CardState::Closed => print!("Zugedeckt: "),
            }
            self.deck.borrow().print_card(card);
        }
        println!();
    }

    /// Methode, um die Karten auf dem Tisch auszugeben
    pub fn print_table(&self) {
        println!("Karten auf dem Tisch:");
        for (card, state) in &self.table_cards {
            match state {
                CardState::Open => print!("Aufgedeckt: "),
                CardState::Closed => print!("Zugedeckt: "),
            }
            self.deck.borrow().print_card(card);
        }
        println!();
    }
}

use colored::*; // Für farbige Terminalausgabe
use rand::seq::SliceRandom; // Für das Mischen des Decks

/// Enum für die möglichen Farben der Karten
#[derive(Copy, Clone, Debug)]
enum CardColors {
    Blk,
    Red,
    Grn,
    Oge,
}

/// Struktur zur Repräsentation einer Karte
#[derive(Copy, Clone, Debug)]
pub struct Card {
    value: u8, // Wert der Karte (0 bis 13), 0 -> Jolly
    sign: u8,  // Symbol der Karte als `u8` (0 = J, 1 = ♣, 2 = ♠, 3 = ♥, 4 = ♦)
    color: CardColors,
}

impl Card {
    /// Konstruktor für eine neue Karte
    fn new(id: u8) -> Self {
        if id == 53 {
            // Joker (Jolly) mit spezieller ID
            Self {
                value: 0,
                sign: 0,
                color: CardColors::Blk,
            }
        } else {
            let value = id % 13 + 1; // Werte von 1 (Ass) bis 13 (König)
            let sign = id / 13; // Symbole entsprechend aufteilen
            Self {
                value,
                sign,
                color: CardColors::Blk,
            }
        }
    }

    /// Gibt eine eindeutige ID für die Karte zurück
    fn get_id(&self) -> u8 {
        if self.value == 0 && self.sign == 0 {
            53 // Joker-ID
        } else {
            self.sign * 13 + (self.value - 1)
        }
    }

}

/// Enum zur Auswahl des Kartendeck-Typs
#[allow(dead_code)]
pub enum DeckType {
    French,
    FourC,
}

/// Struktur zur Repräsentation eines Kartendecks
pub struct Deck {
    cards: Vec<Card>, // Vektor, der alle Karten des Decks und ihre Farben enthält
    deck_type: DeckType,
    deck_size: u8,
    n_joker: u8,
}

impl Deck {
    /// Konstruktor für ein neues Deck
    pub fn new(deck_type: DeckType, deck_size: u8, n_joker: u8) -> Self {
        let _cards = Self::deck(deck_size, n_joker);
        let deck_colors_fn = match deck_type {
            DeckType::French => Self::deck_fr,
            DeckType::FourC => Self::deck_4c,
        };

        let cards = _cards
            .into_iter()
            .map(|id| {
                let mut card = Card::new(id);
                card.color = deck_colors_fn(card.sign);
                card
            })
            .collect();

        Self {
            cards,
            deck_type,
            deck_size,
            n_joker,
        }
    }

    /// Mischt die Karten im Deck zufällig
    #[allow(dead_code)]
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    /// Methode, um eine Karte aus dem Deck zu ziehen
    #[allow(dead_code)]
    pub fn pull(&mut self) -> Option<Card> {
        if let Some(card) = self.cards.pop() {
            Some(card) // Gib die gezogene Karte zurück
        } else {
            None // Kein Ziehen möglich, wenn das Deck leer ist
        }
    }

    /// Methode, um das Deck zurückzusetzen
    #[allow(dead_code)]
    pub fn reset(&mut self) {
        let _cards = Self::deck(self.deck_size, self.n_joker);
        let deck_colors_fn = match self.deck_type {
            DeckType::French => Self::deck_fr,
            DeckType::FourC => Self::deck_4c,
        };

        self.cards = _cards
            .into_iter()
            .map(|id| {
                let mut card = Card::new(id);
                card.color = deck_colors_fn(card.sign);
                card
            })
            .collect();
    }

    /// Hilfsfunktion zur Erstellung des Decks
    fn deck(len: u8, n_jollys: u8) -> Vec<u8> {
        let mut d: Vec<u8> = Vec::new();

        match len {
            32 => {
                // Skat-Deck: Nur Kartenwerte von 7 bis Ass
                for sign in 0..4 {
                    for value in 6..=12 {
                        // Werte von 7 bis Ass (Index: 6 bis 12)
                        d.push(sign * 13 + value);
                    }
                }
            }
            52 => {
                // Standard-Deck: Alle Karten von 0 bis 51 (0 bis 51 sind alle Karten)
                d.extend(0..52);
            }
            72 => {
                // 72er Deck: Zwei Sets von 36 Karten (Kartenwerte 7 bis Ass)
                for _ in 0..2 {
                    for sign in 0..4 {
                        for value in 6..=12 {
                            // Werte von 7 bis Ass (Index: 6 bis 12)
                            d.push(sign * 13 + value);
                        }
                    }
                }
            }
            108 => {
                // 108er Deck: Zwei komplette Sets von 52 Karten
                for _ in 0..2 {
                    d.extend(0..52);
                }
            }
            _ => panic!("Ungültige Deckgröße! Nur 32, 52, 72 oder 108 erlaubt."),
        }

        // Füge die gewünschte Anzahl von Jokern (Jollys) hinzu, ID = 53
        for _ in 0..n_jollys {
            d.push(53);
        }

        d
    }

    /// Funktion zur Definition eines 2-farbigen Blatts
    fn deck_fr(sign: u8) -> CardColors {
        match sign {
            0 | 1 => CardColors::Blk,
            2 | 3 => CardColors::Red,
            _ => panic!("Ungültige Farbe: {}", sign),
        }
    }

    /// Funktion zur Definition eines 4-farbigen Blatts
    fn deck_4c(sign: u8) -> CardColors {
        match sign {
            0 => CardColors::Blk,
            1 => CardColors::Grn,
            2 => CardColors::Red,
            3 => CardColors::Oge,
            _ => panic!("Ungültige Farbe: {}", sign),
        }
    }

    pub fn print_card(&self, card: &Card) {
        let card: ColoredString = Self::card(&self, card);
        print!(" {} ", card);
    }

    pub fn card(&self, card: &Card) -> ColoredString {
        let colored_color = match card.color {
            CardColors::Blk => Color::Black,
            CardColors::Red => Color::Red,
            CardColors::Grn => Color::Green,
            CardColors::Oge => Color::Yellow,
        };

        let sign = if card.get_id() == 53 {
            'J' // Joker-Symbol
        } else {
            match card.sign {
                0 => '♣',
                1 => '♠',
                2 => '♥',
                3 => '♦',
                _ => '?',
            }
        };

        let value_str = if card.get_id() == 53 {
            "J".to_string() // Joker-Wert
        } else {
            match card.value {
                1 => "A".to_string(),
                11 => "J".to_string(),
                12 => "D".to_string(),
                13 => "K".to_string(),
                _ => card.value.to_string(),
            }
        };

        let card_output: ColoredString = format!(" {}{:<2}", sign, value_str)
            .on_white()
            .color(colored_color);

        return card_output;
    }

    /// Gibt das gesamte Deck im Terminal aus
    pub fn print_deck(&self) {
        self.cards.iter().for_each(|card| {
            // print!("ID: {} ", card.get_id());
            self.print_card(card);
        });
        println!(" ");
    }
}

#![allow(private_interfaces)]

use rand::prelude::*;

mod looks;

pub const HAND_SIZE: usize = 5;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Bonus {
    Beach,
    Culture,
    Trekking,
    Wildlife,
}

impl Bonus {
    fn parse(input: &char) -> Self {
        match input {
            'b' => Self::Beach,
            'c' => Self::Culture,
            't' => Self::Trekking,
            'w' => Self::Wildlife,
            _ => panic!("Invalid bonus char -> {}", input),
        }
    }

    fn unparse(&self) -> char {
        match self {
            Self::Beach => 'b',
            Self::Culture => 'c',
            Self::Trekking => 't',
            Self::Wildlife => 'w',
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
enum Continent {
    Africa,
    America,
    Antarctica,
    Asia,
    Europe,
    Oceania,
}

#[derive(Debug, Clone)]
pub struct Country {
    name: String,
    score: u8,
    allowed_bonus: String,
    pub bonus: Vec<Bonus>
}

impl Country {
    fn new(name: &str, score: u8, allowed_bonuses: &str) -> Self {
        Self {
            name: name.to_string(),
            score,
            allowed_bonus: allowed_bonuses.to_string(),
            bonus: Vec::new(),
        }
    }

    fn continent(&self) -> Continent {
        match self.name.as_str() { 
            "Mali" | "Egypt" | "Kenya" | "Morocco" | "Uganda" | "South Africa" | "Zimbabwe" => Continent::Africa,
            "Bolivia"| "Brazil"| "Peru"| "Mexico"| "Argentina"| "USA"| "Canada" => Continent::America,
            "Antarctica" => Continent::Antarctica,
            "Mongolia"| "China"| "India"| "Indonesia"| "Nepal"| "Uzbekistan"| "Thailand"| "Vietnam"| "Japan" => Continent::Asia,
            "Russia"| "Turkey"| "Italy"| "Germany"| "Ireland"| "UK"| "France"| "Holland" => Continent::Europe,
            "Easter Island" | "Tahiti" | "New Zealand" | "Australia" | "Cook Islands" | "Fiji" => Continent::Oceania,
            _ => Continent::Antarctica
        }
    }
}

impl PartialEq for Country {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}


#[derive(Debug, PartialEq, Eq)]
enum AdviceType {
    Money,
    Bureaucracy,
    Timing,
    Transport,
}

#[derive(Debug)]
struct Advice {
    good: bool,
    variant: AdviceType,
}

impl Advice {
    fn new(good: bool, variant: AdviceType) -> Self {
        Self { good, variant }
    }
}


#[derive(Debug)]
enum Special {
    CerditCard,
}





#[derive(Debug)]
pub enum Card {
    Country(Country),
    Bonus(Bonus),
    Advice(Advice),
    Special(Special)
}

impl Card {
    pub fn is_country(&self) -> bool {
        matches!(self, Card::Country(..))
    }

    pub fn country(&self) -> Option<&Country> {
        if let Card::Country(country) = self {
            Some(country)
        } else {
            None
        }
    }

    pub fn is_bonus(&self) -> bool {
        matches!(self, Card::Bonus(..))
    }

    pub fn bonus(&self) -> Option<&Bonus> {
        if let Card::Bonus(bonus) = self {
            Some(bonus)
        } else {
            None
        }
    }
}


#[derive(Debug)]
enum Status {
    MissGo(u8),
    NoCountries(u8), 
    BadAdvice(AdviceType),
    VisaProblem,
}


#[derive(Debug)]
pub struct Player {
    pub hand: Vec<Card>,
    pub pile: Vec<Country>,
    score: u32,
    status: Option<Status>,
}

impl Player {
    fn from_hand(hand: Vec<Card>) -> Self {
        Self {
            hand,
            pile: vec![],
            score: 0,
            status: None,
        }
    }

    pub fn top_country(&self) -> Option<&Country> {
        self.pile.last()
    }

    pub fn top_country_mut(&mut self) -> Option<&mut Country> {
        self.pile.last_mut()
    }

    pub fn can_play_country(&self, country: &Country) -> bool {
        let continent = country.continent();
        
        let times_visited = self.pile.iter().filter(|played| played.continent() == continent).count();
        let have_credit_card = self.hand.iter().any(|card| matches!(card, Card::Special(Special::CerditCard)));
        
        if !have_credit_card && times_visited > 0 { return false; }
        
        if have_credit_card {
            times_visited < 2
        } else {
            times_visited < 1
        }
    }

    pub fn play_country(&mut self, card_index: usize) -> Result<(), String> {
        if card_index >= self.hand.len() {
            return Err("Invalid index".to_string());
        }

        let card = self.hand.swap_remove(card_index);

        if let Card::Country(country) = card {
            if self.can_play_country(&country) {
                self.pile.push(country);
                Ok(())
            } else {
                self.hand.push(Card::Country(country));
                Err("Cant play this country".to_string())
            }
        } else {
            self.hand.push(card);
            Err("Not a country card".to_string())
        }
    }

    pub fn can_play_bonus(&self, bonus: &Bonus) -> bool {
        if let Some(top_country) = self.top_country() && top_country.allowed_bonus.contains(bonus.unparse()) {
            true
        } else {
            false
        }
    }

    pub fn play_bonus(&mut self, card_index: usize) -> Result<(), String> {
        if card_index >= self.hand.len() {
            return Err("Invalid index".to_string());
        }

        let card = self.hand.swap_remove(card_index);

        if let Card::Bonus(bonus) = card {
            if let Some(top_country) = self.top_country_mut() {
                if top_country.allowed_bonus.contains(bonus.unparse()) {
                    top_country.bonus.push(bonus);
                    Ok(())
                } else {
                    self.hand.push(Card::Bonus(bonus));
                    Err("Cant play this bonus on the top country".to_string())
                }
            } else {
                self.hand.push(Card::Bonus(bonus));
                Err("No country on played pile".to_string())
            }
        } else {
            self.hand.push(card);
            Err("Not a bonus card".to_string())
        }
    }
}





pub struct Board {
    future: Vec<Card>,
    past: Vec<Card>,
    pub players: Vec<Player>,
    turn: usize,
}

impl Board {
    pub fn new_game(num_players: usize) -> Self {
        let mut deck = Card::deck();

        let mut rng = rand::rng();
        deck.shuffle(&mut rng);

        let num_held_cards = HAND_SIZE * num_players;

        if num_held_cards >= deck.len() {
            panic!("Too many players / Not enough cards! Players: {}, Cards: {}", num_players, deck.len())
        }


        let mut deck_iter = deck.into_iter();
        let mut to_be_held = deck_iter.by_ref().take(num_held_cards);

        let players: Vec<Player> = (0..num_players)
            .map(|_| {
                let hand: Vec<Card> = to_be_held.by_ref().take(HAND_SIZE).collect();
                Player::from_hand(hand)
            })
            .collect();

        let future: Vec<Card> = deck_iter.collect();
        let past = vec![];


        println!("--- Game Started ---");
        println!("  Players: {}", players.len());
        println!("  Hand Size: {}", HAND_SIZE);
        println!("  Future len: {}", future.len());
        println!();

        Self {
            future,
            past,
            players,
            turn: 0,
        }
    }

    pub fn manual_game(&mut self) {
        self.turn_heading();
    }
}


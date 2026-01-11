#![allow(dead_code)]

use std::{collections::HashMap};
use rand::prelude::*;

mod looks;

pub const HAND_SIZE: usize = 60;


#[derive(Debug, PartialEq, Eq, Hash)]
enum Bonus {
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

#[derive(Debug)]
struct Country {
    name: String,
    score: u8,
    bonus: HashMap<Bonus, u8>,
}

impl Country {
    fn new(name: &str, score: u8, allowed_bonuses: &str) -> Self {
        let mut bonus = HashMap::new();
        allowed_bonuses.chars().for_each(|b| { bonus.insert(Bonus::parse(&b),0); } );

        Self {
            name: name.to_string(),
            score,
            bonus,
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
}


#[derive(Debug)]
enum Status {
    MissGo(u8),
    NoCountries(u8), 
    BadAdvice(AdviceType),
    VisaProblem,
}


#[derive(Debug)]
pub struct Player<'a> {
    pub hand: Vec<Card>,
    pub pile: Vec<&'a Country>,
    score: u32,
    status: Option<Status>,
}

impl<'a> Player<'a> {
    fn from_hand(hand: Vec<Card>) -> Self {
        Self {
            hand,
            pile: vec![],
            score: 0,
            status: None,
        }
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
}





pub struct Board<'a> {
    future: Vec<Card>,
    past: Vec<Card>,
    pub players: Vec<Player<'a>>,
}

impl<'a> Board<'a> {
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
        }
    }
}


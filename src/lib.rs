#![allow(private_interfaces)]

use std::{error::Error, fmt, io, thread::sleep, time::Duration};

use rand::prelude::*;
use text_io::try_read;

mod looks;

const HAND_SIZE: usize = 5;
const PAUSE_TIME: u64 = 700;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
struct Country {
    name: String,
    score: u8,
    allowed_bonus: String,
    bonus: Vec<Bonus>,
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
            "Mali" | "Egypt" | "Kenya" | "Morocco" | "Uganda" | "South Africa" | "Zimbabwe" => {
                Continent::Africa
            }
            "Bolivia" | "Brazil" | "Peru" | "Mexico" | "Argentina" | "USA" | "Canada" => {
                Continent::America
            }
            "Antarctica" => Continent::Antarctica,
            "Mongolia" | "China" | "India" | "Indonesia" | "Nepal" | "Uzbekistan" | "Thailand"
            | "Vietnam" | "Japan" => Continent::Asia,
            "Russia" | "Turkey" | "Italy" | "Germany" | "Ireland" | "UK" | "France" | "Holland" => {
                Continent::Europe
            }
            "Easter Island" | "Tahiti" | "New Zealand" | "Australia" | "Cook Islands" | "Fiji" => {
                Continent::Oceania
            }
            _ => Continent::Antarctica,
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

#[derive(Debug, Clone, Copy)]
enum GreyType {
    MissedFlight,
}

#[derive(Debug)]
enum Special {
    CerditCard,
}

#[derive(Debug)]
enum Card {
    Country(Country),
    Bonus(Bonus),
    Advice(Advice),
    Special(Special),
    Grey(GreyType),
}

impl Card {
    fn is_country(&self) -> bool {
        matches!(self, Card::Country(..))
    }

    fn country(&self) -> Option<&Country> {
        if let Card::Country(country) = self {
            Some(country)
        } else {
            None
        }
    }

    fn is_bonus(&self) -> bool {
        matches!(self, Card::Bonus(..))
    }

    fn bonus(&self) -> Option<&Bonus> {
        if let Card::Bonus(bonus) = self {
            Some(bonus)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum StatusType {
    // Player will miss their next go(s)
    MissGo(u8),
    // ??
    NoCountries(u8),
    // Affected by Bad Advice
    BadAdvice(AdviceType),
    // Affected by Visa Problem
    VisaProblem,
}

#[derive(Debug)]
struct StatusHandler {
    types: Vec<StatusType>,
}

impl StatusHandler {
    fn empty() -> Self {
        Self { types: Vec::new() }
    }

    fn add_status(&mut self, status: StatusType) {
        self.types.push(status);
    }

    fn remove_status(&mut self, status: StatusType) {
        self.types.retain(|t| *t != status);
    }

    fn no_turn(&mut self) -> bool {
        for ty in self.types.iter_mut() {
            if let StatusType::MissGo(gos) = ty {
                if gos > &mut 1 {
                    println!("Missing this go. {} more to go", gos);
                } else {
                    println!("Missing this go.");
                }
                *gos -= 1;

                self.cleanup();

                sleep(Duration::from_millis(PAUSE_TIME));
                return true;
            }
        }

        sleep(Duration::from_millis(PAUSE_TIME));
        false
    }

    fn cleanup(&mut self) {
        self.types.retain(|ty| match ty {
            StatusType::MissGo(gos) => *gos != 0,
            _ => true,
        });
    }
}

#[derive(Debug)]
enum BError {
    Custom(String),
    SameContinent,
    GreyHeld,
}

impl Error for BError {}

impl fmt::Display for BError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(string) => write!(f, "{}", string),
            Self::SameContinent => write!(f, "too many countries of the same continent"),
            Self::GreyHeld => write!(f, "you can't go home with grey cards"),
        }
    }
}

#[derive(Debug)]
pub struct Player {
    hand: Vec<Card>,
    pile: Vec<Country>,
    score: u32,
    status: StatusHandler,
    temp: Option<Card>,
}

impl Player {
    fn from_hand(hand: Vec<Card>) -> Self {
        Self {
            hand,
            pile: vec![],
            score: 0,
            status: StatusHandler::empty(),
            temp: None,
        }
    }

    fn top_country(&self) -> Option<&Country> {
        self.pile.last()
    }

    fn top_country_mut(&mut self) -> Option<&mut Country> {
        self.pile.last_mut()
    }

    pub fn add_status(&mut self, status: StatusType) {
        self.status.add_status(status);
    }

    fn can_go_home(&self) -> bool {
        self.hand.iter().any(|card| matches!(card, Card::Grey(_)))
    }

    fn go_home(&mut self) -> Result<(), BError> {
        if !self.can_go_home() {
            return Err(BError::GreyHeld);
        }

        self.score += self
            .pile
            .iter()
            .map(|card| {
                let Country {
                    name: _,
                    score,
                    allowed_bonus: _,
                    bonus,
                } = card;
                *score as u32 * (1 + bonus.len() as u32)
            })
            .sum::<u32>();

        Ok(())
    }

    fn can_play_country(&self, country: &Country) -> Result<(), BError> {
        let continent = country.continent();

        let times_visited = self
            .pile
            .iter()
            .filter(|played| played.continent() == continent)
            .count();
        let have_credit_card = self
            .hand
            .iter()
            .any(|card| matches!(card, Card::Special(Special::CerditCard)));

        if have_credit_card {
            if times_visited >= 2 {
                return Err(BError::SameContinent);
            }
        } else if times_visited >= 1 {
            return Err(BError::SameContinent);
        }

        Ok(())
    }

    fn play_country(&mut self, card_index: usize) -> Result<(), BError> {
        if card_index >= self.hand.len() {
            // return Err("Invalid index".to_string());
            panic!("This should be checked before calling `play_country`");
        }

        let card = self.hand.swap_remove(card_index);

        if let Card::Country(country) = card {
            if let Err(err) = self.can_play_country(&country) {
                self.hand.push(Card::Country(country));
                Err(err)
            } else {
                self.pile.push(country);
                Ok(())
            }
        } else {
            self.hand.push(card);
            // Err("Not a country card".to_string())
            panic!("This should be checked before calling `play_country`");
        }
    }

    fn can_play_bonus(&self, bonus: &Bonus) -> bool {
        if let Some(top_country) = self.top_country()
            && top_country.allowed_bonus.contains(bonus.unparse())
        {
            true
        } else {
            false
        }
    }

    fn play_bonus(&mut self, card_index: usize) -> Result<(), String> {
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

    fn play_grey(&mut self, card_index: usize) -> Result<(), BError> {
        if card_index >= self.hand.len() {
            return Err(BError::Custom("Invalid index".to_string()));
        }

        let card = self.hand.swap_remove(card_index);

        if let Card::Grey(grey) = card {
            match grey {
                GreyType::MissedFlight => {
                    self.add_status(StatusType::MissGo(1));
                    self.temp = Some(card);
                    Ok(())
                }
            }
        } else {
            self.hand.push(card);
            Err(BError::Custom("Not a grey card".to_string()))
        }
    }
}

fn read_line() -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

fn get_requested_input<T, F>(message: &str, condition: F) -> T
where
    T: PartialOrd + std::str::FromStr<Err: std::fmt::Debug>,
    F: Fn(&T) -> bool,
{
    let mut output = None;
    while output.is_none() {
        println!("{}", message);
        let inp_opt = try_read!();

        match inp_opt {
            Ok(inp) => {
                if condition(&inp) {
                    output = Some(inp);
                } else {
                    println!("Invalid value\n");
                }
            }
            Err(_) => println!("Error reading input\n"),
        }
    }

    output.unwrap()
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
            panic!(
                "Too many players / Not enough cards! Players: {}, Cards: {}",
                num_players,
                deck.len()
            )
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

    fn next_turn(&mut self) {
        if self.turn == self.players.len() - 1 {
            self.turn = 0;
        } else {
            self.turn += 1;
        }
    }

    fn discard(&mut self, card: Card) {
        self.past.push(card);
    }

    fn player_discard(&mut self, card_index: usize) {
        let card = self.players[self.turn].hand.swap_remove(card_index);
        self.discard(card);
    }

    pub fn manual_game(&mut self) {
        while !self.future.is_empty() {
            self.turn_heading();
            self.manual_turn();
        }
    }

    fn manual_turn(&mut self) {
        // match get_requested_input("Go home?: ", 1) {
        //     1 => self.players[self.turn].go_home(),
        // }

        let mut finished_turn = false;
        while !finished_turn {
            sleep(Duration::from_millis(PAUSE_TIME));
            let res = self.manual_try_turn();
            match res {
                Ok(_) => finished_turn = true,
                Err(e) => println!("{}", e),
            }
        }
        self.next_turn();
    }

    fn manual_try_turn(&mut self) -> Result<(), BError> {
        let current_player = &mut self.players[self.turn];

        if current_player.status.no_turn() {
            return Ok(());
        }

        let mut selected =
            get_requested_input("Pick a card to play, or 0 to discard, 10 to home", |&inp| {
                inp <= self.players[self.turn].hand.len() || inp == 10
            });

        if selected == 0 {
            let to_discard: usize = get_requested_input("Pick a card to discard", |&inp| {
                inp < self.players[self.turn].hand.len() && inp > 0
            });
            let to_discard = to_discard - 1;
            self.player_discard(to_discard);
            return Ok(());
        } else if selected == 10 {
            self.players[self.turn].go_home()?
        }

        // Allow for 1-based indexing for the user, and for 0 to represent a discard selection
        selected -= 1;
        println!("Selected {}", self.players[self.turn].hand[selected]);

        let out = match &self.players[self.turn].hand[selected] {
            Card::Country(_) => self.manual_play_country(selected),
            Card::Grey(_) => self.manual_play_grey(selected),
            _ => Ok(()),
        };
        sleep(Duration::from_millis(PAUSE_TIME));

        out
    }

    fn manual_play_country(&mut self, card_index: usize) -> Result<(), BError> {
        self.players[self.turn].play_country(card_index)
    }

    fn manual_play_grey(&mut self, card_index: usize) -> Result<(), BError> {
        self.players[self.turn].play_grey(card_index)
    }
}

use std::{thread::sleep, time::Duration};

use colored::{Colorize, CustomColor};
use rand::seq::SliceRandom;

use crate::{HAND_SIZE, PAUSE_TIME, cards::card::Card, player::Player, utils::BError};

pub(crate) enum BoardMove {
    PlayCard(usize),
}

pub(crate) struct Board {
    future: Vec<Card>,
    past: Vec<Card>,
    pub(crate) players: Vec<Player>,
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

    pub(crate) fn current_turn(&self) -> usize {
        self.turn
    }

    fn next_turn(&mut self) {
        if self.turn == self.players.len() - 1 {
            self.turn = 0;
        } else {
            self.turn += 1;
        }
    }

    fn end_turn(&mut self) {
        self.players[self.turn].sort_hand();
        self.next_turn();
    }

    fn discard(&mut self, card: Card) {
        self.past.push(card);
    }

    fn player_discard(&mut self, card_index: usize) {
        let card = self.players[self.turn].swap_remove(card_index);
        self.discard(card);
    }

    pub(crate) fn make_move(&mut self, action: BoardMove) -> Result<(), BError> {
        let result = match action {
            BoardMove::PlayCard(card_index) => self.play_card(card_index),
        };

        if result.is_ok() {
            self.end_turn();
        }

        result
    }

    fn play_card(&mut self, card_index: usize) -> Result<(), BError> {
        if card_index >= self.players[self.turn].hand_len() {
            return Err(BError::Custom("Invalid card index".to_string()));
        }

        match self.players[self.turn].get(card_index) {
            Card::Bonus(_) => self.players[self.turn].play_bonus(card_index),
            Card::Country(_) => self.players[self.turn].play_country(card_index),
            Card::Grey(_) => self.players[self.turn].play_grey(card_index),
            _ => Err(BError::Custom(
                "That card cannot be played from the hand right now".to_string(),
            )),
        }
    }

    pub(crate) fn manual_game(&mut self) {
        while !self.future.is_empty() {
            self.turn_heading();
            self.manual_turn();
        }
    }

    pub(crate) fn manual_turn(&mut self) {
        if self.players[self.turn].no_turn() {
            return;
        }

        if 1 == crate::utils::get_requested_input("Go home?: ", |_| true) {
            match self.players[self.turn].go_home() {
                Ok(mut cards) => {
                    self.past.append(&mut cards);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
            self.end_turn();
            sleep(Duration::from_millis(PAUSE_TIME));
        } else {
            let mut finished_turn = false;
            while !finished_turn {
                sleep(Duration::from_millis(PAUSE_TIME));
                let res = self.manual_try_turn();
                match res {
                    Ok(_) => finished_turn = true,
                    Err(e) => println!("{}", e),
                }
            }
        }
    }

    fn manual_try_turn(&mut self) -> Result<(), BError> {
        let mut selected = crate::utils::get_requested_input(
            "Pick a card to play, or 0 to discard, 10 to home",
            |&inp| inp <= self.players[self.turn].hand_len() || inp == 10,
        );

        if selected == 0 {
            let to_discard: usize =
                crate::utils::get_requested_input("Pick a card to discard", |&inp| {
                    inp < self.players[self.turn].hand_len() && inp > 0
                });
            let to_discard = to_discard - 1;
            self.player_discard(to_discard);
            return Ok(());
        }

        // Allow for 1-based indexing for the user, and for 0 to represent a discard selection
        selected -= 1;
        println!("Selected {}", self.players[self.turn].get(selected));

        let out = match self.players[self.turn].get(selected) {
            Card::Bonus(_) | Card::Country(_) | Card::Grey(_) => {
                self.make_move(BoardMove::PlayCard(selected))
            }
            _ => Err(BError::Custom(
                "That card cannot be played from the hand right now".to_string(),
            )),
        };
        sleep(Duration::from_millis(PAUSE_TIME));

        out
    }

    pub fn turn_heading(&self) {
        println!();
        println!("--------------------------");
        println!("Its player {}'s turn", self.turn + 1);

        // Current player's hand
        println!("Player {}'s hand:", self.turn + 1);
        self.players[self.turn]
            .iter_hand()
            .enumerate()
            .for_each(|(i, card)| println!("| {} {}", i + 1, card));

        println!();

        // All player's played piles
        for (i, player) in self.players.iter().enumerate() {
            for (j, card) in player.iter_pile().enumerate() {
                let bonus_text = if let Some(country) = player.top_country() {
                    country
                        .allowed_bonus
                        .to_uppercase()
                        .custom_color(CustomColor::new(106, 229, 218))
                } else {
                    "".custom_color(CustomColor::new(106, 229, 218))
                };

                if j == 0 {
                    println!("Player {}", i)
                };
                println!("| {} - {}", card, bonus_text);
                for bonus in card.bonus.iter() {
                    println!("| ↳ {}", bonus)
                }
            }
        }

        println!();

        // maybe use this somewhere
        // use tabular::{Table, Row};

        // let mut row_spec = String::new();
        // for _ in 0..self.players.len() {
        //     row_spec.push_str("| {:<}   ");
        // }

        // let mut table = Table::new(&row_spec);

        // let mut row = Row::new();
        // for i in 0..self.players.len() {
        //     row.add_cell(&format!("Player {}", i));
        // }
        // table.add_row(row);

        // for i in 0..self.players.iter().map(|p| p.pile.len()).max().unwrap_or(0) {
        //     let mut row = Row::new();
        //     for player in &self.players {
        //         let row_text = if let Some(country) = player.pile.get(i) {
        //             format!("{}", country)
        //         } else {
        //             "".to_string()
        //         };
        //         row.add_cell(row_text);
        //     }
        //     println!("{}", row.len());
        //     table.add_row(row);
        // }

        // println!("{}", table);
    }
}

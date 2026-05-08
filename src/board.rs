// use std::{thread::sleep, time::Duration};

use rand::seq::SliceRandom;
// use rand::seq::IteratorRandom;

use crate::{HAND_SIZE, cards::card::Card, player::Player, utils::BError};

// use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Display, PartialEq)]
pub(crate) enum TurnStage {
    ChooseGoHome,
    PlayOrDiscard,
}

#[derive(EnumIter, Debug)]
pub(crate) enum PlayerAction {
    GoHome(bool),
    Play(usize),
    Discard(usize),
}

// Sorted by importance / in the order they should be considered.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TurnEffect {
    PassCardLeft,
    EndTurn,
}

pub(crate) struct Board {
    pub(crate) future: Vec<Card>,
    pub(crate) past: Vec<Card>,
    pub(crate) players: Vec<Player>,
    pub(crate) turn: usize,
    pub(crate) turn_stage: TurnStage,
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
            turn_stage: TurnStage::ChooseGoHome,
        }
    }

    fn start_turn(&mut self) {
        if self.players[self.turn].start_turn() {
            self.next_turn();
            return;
        }
        self.turn_stage = TurnStage::ChooseGoHome;
    }

    pub(crate) fn apply_action(&mut self, action: PlayerAction) -> Result<(), BError> {
        println!("{:?}", action);
        match self.turn_stage {
            TurnStage::ChooseGoHome => {
                if !matches!(action, PlayerAction::GoHome(..)) {
                    return Err(BError::PlayedBeforeHomeChoice);
                }
            }
            TurnStage::PlayOrDiscard => {
                if !(matches!(action, PlayerAction::Play(..))
                    || matches!(action, PlayerAction::Discard(..)))
                {
                    return Err(BError::HomeChoiceMissed);
                }
            }
        }

        let effects = self.resolve_action(action)?;
        self.apply_effects(effects);
        Ok(())
    }

    fn resolve_action(&mut self, action: PlayerAction) -> Result<Vec<TurnEffect>, BError> {
        let mut effects = vec![];
        match action {
            PlayerAction::GoHome(go_home) => {
                if go_home {
                    // Get cards to be added to the past pile.
                    let mut cards = self.players[self.turn].go_home()?;
                    // Discard them
                    self.past.append(&mut cards);
                    // Signal the end of the turn.
                    effects.push(TurnEffect::EndTurn);
                } else {
                    // Player chose not to go home, so move onto playing / discarding.
                    self.turn_stage = TurnStage::PlayOrDiscard;
                }
            }
            PlayerAction::Play(card_index) => {
                let new_effects = self.play_card(card_index)?;
                effects.extend(new_effects);
            }
            PlayerAction::Discard(card_index) => self.player_discard(card_index)?,
        }

        effects.sort();
        Ok(effects)
    }

    fn apply_effects(&mut self, effects: Vec<TurnEffect>) {
        for effect in effects {
            println!("Applying effect: {:?}", effect);
            match effect {
                TurnEffect::PassCardLeft => self.pass_card_left(),
                TurnEffect::EndTurn => self.end_turn(),
            }
        }
    }

    fn end_turn(&mut self) {
        self.players[self.turn].sort_hand();
        self.next_turn();
    }

    fn next_turn(&mut self) {
        if self.turn == self.players.len() - 1 {
            self.turn = 0;
        } else {
            self.turn += 1;
        }

        self.start_turn();
    }

    fn discard(&mut self, card: Card) {
        self.past.push(card);
    }

    fn player_discard(&mut self, card_index: usize) -> Result<(), BError> {
        let card = self.players[self.turn].swap_remove(card_index);
        if card.is_grey() {
            return Err(BError::FreeGreyDiscard);
        }
        self.discard(card);
        Ok(())
    }

    fn pass_card_left(&mut self) {
        todo!("Not sure how to do this yet. This will be a problemo");
    }

    fn play_card(&mut self, card_index: usize) -> Result<Vec<TurnEffect>, BError> {
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

    // pub(crate) fn make_random_move(&mut self) {
    //     if let Some(bm) = BoardMove::iter().choose(&mut rand::rng()) {
    //         match bm {
    //             BoardMove::PlayCard(_) => {
    //                 let hand_len = self.players[self.turn].hand_len();
    //                 if hand_len == 0 {
    //                     self.skip_turn();
    //                     return;
    //                 }

    //                 let index = rand::Rng::random_range(
    //                     &mut rand::rng(),
    //                     0..self.players[self.turn].hand_len(),
    //                 );
    //                 if self.make_move(BoardMove::PlayCard(index)).is_ok() {
    //                     return;
    //                 }
    //             }
    //         }
    //     }

    //     self.skip_turn();
    // }
}

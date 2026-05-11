// use std::{thread::sleep, time::Duration};

use rand::seq::SliceRandom;
// use rand::seq::IteratorRandom;

use crate::{
    HAND_SIZE, NUM_PLAYERS,
    cards::{
        bonus::Bonus,
        card::{Card, RenderableCard},
    },
    player::Player,
    state::{CardView, CountryView, GameSnapshot, PlayerView},
    utils::BError,
};

// use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

/// The current stage of the current player's turn
#[derive(Debug, Display, PartialEq)]
pub enum TurnStage {
    ChooseGoHome,
    PlayOrDiscard,
}

/// The action a player can take on their turn. This is the input to the `apply_action` function, which will perform the action, resolve it into one or more `TurnEffect`s.
#[derive(EnumIter, Debug)]
pub enum PlayerAction {
    GoHome(bool),
    Play(usize),
    Discard(usize),
}

/// The game-wide effects of a player's action.
/// Sorted by importance / in the order they should be considered.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TurnEffect {
    PassCardLeft,
    EndTurn,
}

pub struct Board {
    /// The future pile - The cards that are yet to be drawn.
    pub future: Vec<Card>,
    /// The past pile - The cards that have been discarded or played.
    pub past: Vec<Card>,
    /// The `Player`s in the game, in turn order.
    pub players: Vec<Player>,
    /// The index of the current player's turn in the `players` vector.
    pub turn: usize,
    /// The current stage of the current player's turn which requires input from the player.
    pub turn_stage: TurnStage,
}

impl Board {
    pub fn new_game() -> Self {
        let mut deck = Card::deck();

        let mut rng = rand::rng();
        deck.shuffle(&mut rng);

        let num_held_cards = HAND_SIZE * NUM_PLAYERS;

        if num_held_cards >= deck.len() {
            panic!(
                "Too many players / Not enough cards! Players: {}, Cards: {}",
                NUM_PLAYERS,
                deck.len()
            )
        }

        let mut deck_iter = deck.into_iter();
        let mut to_be_held = deck_iter.by_ref().take(num_held_cards);

        let players: Vec<Player> = (0..NUM_PLAYERS)
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

    pub fn apply_action(&mut self, action: PlayerAction) -> Result<(), BError> {
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
                    // Draw a card
                    self.players[self.turn].pick_up(self.future.pop().unwrap());
                    // Move to the next stage of the turn.
                    self.turn_stage = TurnStage::PlayOrDiscard;
                }
            }
            PlayerAction::Play(card_index) => {
                let new_effects = self.play_card(card_index)?;
                effects.extend(new_effects);
            }
            PlayerAction::Discard(card_index) => {
                let new_effects = self.player_discard(card_index)?;
                effects.extend(new_effects);
            }
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

    fn player_discard(&mut self, card_index: usize) -> Result<Vec<TurnEffect>, BError> {
        let card = self.players[self.turn].swap_remove(card_index);
        if card.is_grey() {
            panic!(
                "Grey cards should not be discardable! This should be checked before calling `player_discard`"
            );
        }
        self.discard(card);
        Ok(vec![TurnEffect::EndTurn])
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

    pub fn snapshot(&self) -> GameSnapshot {
        GameSnapshot {
            turn: self.turn,
            turn_stage: self.turn_stage.to_string(),
            future: self.future.iter().map(card_view).collect(),
            past: self.past.iter().map(card_view).collect(),
            players: self
                .players
                .iter()
                .map(|player| PlayerView {
                    hand: player.iter_hand().map(card_view).collect(),
                    pile: player.iter_pile().map(country_view).collect(),
                    score: player.score,
                })
                .collect(),
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

fn card_view(card: &Card) -> CardView {
    let (label, color) = card.render_info();
    CardView {
        label,
        color: [color.r(), color.g(), color.b(), color.a()],
        is_grey: card.is_grey(),
    }
}

fn country_view(country: &crate::cards::country::Country) -> CountryView {
    let (label, color) = country.render_info();
    CountryView {
        label,
        color: [color.r(), color.g(), color.b(), color.a()],
        bonuses: country
            .bonus
            .iter()
            .map(|bonus: &Bonus| card_view(&Card::Bonus(*bonus)))
            .collect(),
    }
}

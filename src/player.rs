use crate::{
    board::TurnEffect,
    cards::{bonus::Bonus, card::Card, country::Country, grey::GreyType, special::Special},
    status::{StatusHandler, StatusType},
    utils::BError,
};

#[derive(Debug)]
pub struct Player {
    /// The cards currently in the player's hand.
    hand: Vec<Card>,
    /// The chain of point related cards played by the player.
    pile: Vec<Country>,
    /// The player's current score.
    pub score: u32,
    /// The status effects currently affecting the player.
    status: StatusHandler,
}

impl Player {
    /// Creates a new player with the given hand of cards.
    pub(crate) fn from_hand(hand: Vec<Card>) -> Self {
        let mut player = Self {
            hand,
            pile: vec![],
            score: 0,
            status: StatusHandler::empty(),
        };
        player.sort_hand();
        player
    }

    /// Performs actions that happen before the player has any input, such as missing their go.
    pub(crate) fn start_turn(&mut self) -> bool {
        self.no_turn()
    }

    /// Adds a card to the player's hand.
    pub(crate) fn pick_up(&mut self, card: Card) {
        self.hand.push(card);
    }

    /// Allows access to the accessable top card of the `Player`'s pile, or None if it is empty.
    pub(crate) fn top_country(&self) -> Option<&Country> {
        self.pile.last()
    }

    /// Allows mutable access to the accessable top card of the `Player`'s pile, or None if it is empty.
    pub(crate) fn top_country_mut(&mut self) -> Option<&mut Country> {
        self.pile.last_mut()
    }

    /// Gets the `index`th card in the player's hand.
    pub(crate) fn get(&self, index: usize) -> &Card {
        &self.hand[index]
    }

    /// Returns the number of cards in the player's hand. Really should only be HAND_SIZE (not during a turn)? check this ig
    pub fn hand_len(&self) -> usize {
        self.hand.len()
    }

    /// Adds a status effect to the `Player`.
    pub fn add_status(&mut self, status: StatusType) {
        self.status.add_status(status);
    }

    pub(crate) fn sort_hand(&mut self) {
        self.hand.sort();
    }

    /// Iterates over the `Card`s in the `Player`'s hand. User for display
    pub(crate) fn iter_hand(&self) -> impl Iterator<Item = &Card> {
        self.hand.iter()
    }

    /// Iterates over the `Country`s in the `Player`'s pile. User for display
    pub fn iter_pile(&self) -> impl Iterator<Item = &Country> {
        self.pile.iter()
    }

    pub(crate) fn swap_remove(&mut self, index: usize) -> Card {
        self.hand.swap_remove(index)
    }

    /// Returns true if the player misses their turn, false otherwise. See `StatusHandler::no_turn`.
    pub(crate) fn no_turn(&mut self) -> bool {
        self.status.no_turn()
    }

    // -TODO: doesnt include played attacking cards
    /// Checks if the player can go home. If not, returns an error describing why.
    fn can_go_home(&self) -> Result<(), BError> {
        if self.hand.iter().any(|card| matches!(card, Card::Grey(_))) {
            return Err(BError::GreyHeld);
        }
        Ok(())
    }

    /// Attempts to go home. If the player cannot go home, returns an error describing why. Otherwise, returns the cards that should be discarded.
    pub(crate) fn go_home(&mut self) -> Result<Vec<Card>, BError> {
        self.can_go_home()?;

        let to_add = self.pile.iter().map(|card| card.total_score()).sum::<u32>();

        println!("Adding {} points", to_add);
        self.score += to_add;

        let mut cards = Vec::new();
        for mut country in self.pile.drain(..) {
            country
                .drain_bonus()
                .for_each(|bonus| cards.push(Card::Bonus(bonus)));
            cards.push(Card::Country(country));
        }

        Ok(cards)
    }

    fn can_play_country(&self, country: &Country) -> Result<(), BError> {
        let continent = country.continent();

        let times_visited = self
            .pile
            .iter()
            .filter(|played| played.continent() == continent)
            .count();
        let has_credit_card = self
            .hand
            .iter()
            .any(|card| matches!(card, Card::Special(Special::CreditCard)));

        if has_credit_card {
            if times_visited >= 2 {
                return Err(BError::SameContinent);
            }
        } else if times_visited >= 1 {
            return Err(BError::SameContinent);
        }

        Ok(())
    }

    pub(crate) fn play_country(&mut self, card_index: usize) -> Result<Vec<TurnEffect>, BError> {
        let card = self.hand.swap_remove(card_index);

        if let Card::Country(country) = card {
            if let Err(err) = self.can_play_country(&country) {
                self.hand.push(Card::Country(country));
                Err(err)
            } else {
                println!("Playing {}", &country);
                self.pile.push(country);
                Ok(vec![TurnEffect::EndTurn])
            }
        } else {
            self.hand.push(card);
            panic!("This should be checked before calling `play_country`");
        }
    }

    fn can_play_bonus(&self, bonus: &Bonus) -> Result<(), BError> {
        if let Some(top_country) = self.top_country() {
            if !top_country.allowed_bonus.contains(bonus.unparse()) {
                return Err(BError::InvalidBonus);
            }
        } else {
            return Err(BError::NoTopCountry);
        }

        Ok(())
    }

    pub(crate) fn play_bonus(&mut self, card_index: usize) -> Result<Vec<TurnEffect>, BError> {
        let card = self.hand.swap_remove(card_index);

        if let Card::Bonus(bonus) = card {
            if let Err(err) = self.can_play_bonus(&bonus) {
                self.hand.push(Card::Bonus(bonus));
                Err(err)
            } else {
                let top_country = self.top_country_mut().unwrap();
                println!("Playing {} on {}", &bonus, &top_country);
                top_country.bonus.push(bonus);
                Ok(vec![TurnEffect::EndTurn])
            }
        } else {
            self.hand.push(card);
            panic!("This should be checked before calling `play_country`");
        }
    }

    pub(crate) fn play_grey(&mut self, card_index: usize) -> Result<Vec<TurnEffect>, BError> {
        if card_index >= self.hand.len() {
            return Err(BError::Custom("Invalid index".to_string()));
        }

        let card = self.hand.swap_remove(card_index);

        if let Card::Grey(grey) = card {
            match grey {
                GreyType::MissedFlight => {
                    self.add_status(StatusType::MissGo(1));
                    Ok(vec![TurnEffect::EndTurn])
                }
                GreyType::Sickness => {
                    self.add_status(StatusType::MissGo(1));
                    Ok(vec![TurnEffect::PassCardLeft, TurnEffect::EndTurn])
                }
            }
        } else {
            self.hand.push(card);
            Err(BError::Custom("Not a grey card".to_string()))
        }
    }

    fn _show_pile(&self) {
        print!("[");
        for (i, c) in self.pile.iter().enumerate() {
            if i != 0 {
                print!(", ");
            }
            print!("{}", c);
        }
        println!("]");
    }

    fn _try_playing_all_counties(&mut self) {
        for i in 0..self.hand.len() {
            let _ = self.play_country(i);
            self._try_playing_all_bonuses();
        }
    }

    fn _try_playing_all_bonuses(&mut self) {
        for i in 0..self.hand.len() {
            let _ = self.play_bonus(i);
        }
    }
}

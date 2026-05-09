use crate::{
    board::TurnEffect,
    cards::{bonus::Bonus, card::Card, country::Country, grey::GreyType, special::Special},
    status::{StatusHandler, StatusType},
    utils::BError,
};

#[derive(Debug)]
pub struct Player {
    hand: Vec<Card>,
    pile: Vec<Country>,
    pub score: u32,
    status: StatusHandler,
}

impl Player {
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

    pub(crate) fn start_turn(&mut self) -> bool {
        self.no_turn()
    }

    pub(crate) fn pick_up(&mut self, card: Option<Card>) {
        if let Some(card) = card {
            self.hand.push(card);
        }
    }

    pub(crate) fn top_country(&self) -> Option<&Country> {
        self.pile.last()
    }

    pub(crate) fn top_country_mut(&mut self) -> Option<&mut Country> {
        self.pile.last_mut()
    }

    pub(crate) fn get(&self, index: usize) -> &Card {
        &self.hand[index]
    }

    /// Returns the number of cards in the player's hand. Really should only be 5? check this
    pub fn hand_len(&self) -> usize {
        self.hand.len()
    }

    pub fn add_status(&mut self, status: StatusType) {
        self.status.add_status(status);
    }

    pub(crate) fn sort_hand(&mut self) {
        self.hand.sort();
    }

    pub(crate) fn iter_hand(&self) -> impl Iterator<Item = &Card> {
        self.hand.iter()
    }

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
    fn can_go_home(&self) -> Result<(), BError> {
        if self.hand.iter().any(|card| matches!(card, Card::Grey(_))) {
            return Err(BError::GreyHeld);
        }
        Ok(())
    }

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

use colored::{Colorize, CustomColor};

use crate::cards::{bonus::Bonus, continent::Continent};

#[derive(Debug, Clone, Eq, PartialOrd, Ord)]
pub(crate) struct Country {
    pub(crate) name: String,
    pub(crate) score: u8,
    pub(crate) allowed_bonus: String,
    pub(crate) bonus: Vec<Bonus>,
}

impl Country {
    pub(crate) fn new(name: &str, score: u8, allowed_bonuses: &str) -> Self {
        Self {
            name: name.to_string(),
            score,
            allowed_bonus: allowed_bonuses.to_string(),
            bonus: Vec::new(),
        }
    }

    pub(crate) fn continent(&self) -> Continent {
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

    pub(crate) fn total_score(&self) -> u32 {
        self.score as u32 * (1 + self.bonus.len() as u32)
    }

    pub(crate) fn drain_bonus(&mut self) -> impl Iterator<Item = Bonus> {
        self.bonus.drain(..)
    }
}

impl PartialEq for Country {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl std::fmt::Display for Country {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.continent() {
            Continent::Africa => write!(
                f,
                "{}",
                self.name.custom_color(CustomColor::new(134, 80, 29))
            ),
            Continent::Asia => write!(
                f,
                "{}",
                self.name.custom_color(CustomColor::new(196, 181, 61))
            ),
            Continent::America => write!(
                f,
                "{}",
                self.name.custom_color(CustomColor::new(234, 83, 119))
            ),
            Continent::Antarctica => write!(
                f,
                "{}",
                self.name.custom_color(CustomColor::new(220, 220, 220))
            ),
            Continent::Europe => write!(
                f,
                "{}",
                self.name.custom_color(CustomColor::new(118, 72, 141))
            ),
            Continent::Oceania => write!(
                f,
                "{}",
                self.name.custom_color(CustomColor::new(113, 209, 164))
            ),
        }
    }
}

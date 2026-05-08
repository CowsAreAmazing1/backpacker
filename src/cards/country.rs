use colored::Colorize;

use crate::cards::{bonus::Bonus, card::RenderableCard};

const AFRICA: (u8, u8, u8) = (134, 80, 29);
const ASIA: (u8, u8, u8) = (196, 181, 61);
const AMERICA: (u8, u8, u8) = (234, 83, 119);
const ANTARCTICA: (u8, u8, u8) = (220, 220, 220);
const EUROPE: (u8, u8, u8) = (118, 72, 141);
const OCEANIA: (u8, u8, u8) = (113, 209, 164);

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Continent {
    Africa,
    America,
    Antarctica,
    Asia,
    Europe,
    Oceania,
}

/// Country card struct,
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

    /// Determines the continent of the country based on its name. This is used for repeated continent logic and display coloring.
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

    /// Returns the total score of the country, including all attatched bonuses, given by `score * (1 + bonus_count)`.
    pub(crate) fn total_score(&self) -> u32 {
        self.score as u32 * (1 + self.bonus.len() as u32)
    }

    /// Drains all `Bonus`es from this `Country` which resets the card for discarding, and returns an iterator over the drained bonuses so they can be handled separately.
    pub(crate) fn drain_bonus(&mut self) -> impl Iterator<Item = Bonus> {
        self.bonus.drain(..)
    }

    /// Raw string representation of the `Country` card. -TODO: Does not include any bonus information
    pub(crate) fn raw_display(&self) -> String {
        self.name.to_string()
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
            Continent::Africa => write!(f, "{}", self.name.custom_color(AFRICA)),
            Continent::Asia => write!(f, "{}", self.name.custom_color(ASIA)),
            Continent::America => write!(f, "{}", self.name.custom_color(AMERICA)),
            Continent::Antarctica => write!(f, "{}", self.name.custom_color(ANTARCTICA)),
            Continent::Europe => write!(f, "{}", self.name.custom_color(EUROPE)),
            Continent::Oceania => write!(f, "{}", self.name.custom_color(OCEANIA)),
        }
    }
}

impl RenderableCard for Country {
    fn render_info(&self) -> (String, egui::Color32) {
        let color = match self.continent() {
            Continent::Africa => egui::Color32::from_rgb(AFRICA.0, AFRICA.1, AFRICA.2),
            Continent::Asia => egui::Color32::from_rgb(ASIA.0, ASIA.1, ASIA.2),
            Continent::America => egui::Color32::from_rgb(AMERICA.0, AMERICA.1, AMERICA.2),
            Continent::Antarctica => {
                egui::Color32::from_rgb(ANTARCTICA.0, ANTARCTICA.1, ANTARCTICA.2)
            }
            Continent::Europe => egui::Color32::from_rgb(EUROPE.0, EUROPE.1, EUROPE.2),
            Continent::Oceania => egui::Color32::from_rgb(OCEANIA.0, OCEANIA.1, OCEANIA.2),
        };
        (self.name.to_string(), color)
    }
}

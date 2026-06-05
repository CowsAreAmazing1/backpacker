use colored::Colorize;

use crate::{cards::card::RenderableCard, utils::u8_tup_to_color32};

const GREY: (u8, u8, u8) = (120, 120, 120);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum GreyType {
    MissedFlight,
    Sickness,
}

impl GreyType {}

impl std::fmt::Display for GreyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreyType::MissedFlight => write!(f, "{}", "Missed Flight".custom_color(GREY)),
            GreyType::Sickness => write!(f, "{}", "Sickness".custom_color(GREY)),
        }
    }
}

impl RenderableCard for GreyType {
    fn raw_display(&self) -> String {
        match self {
            GreyType::MissedFlight => "Missed Flight".to_string(),
            GreyType::Sickness => "Sickness".to_string(),
        }
    }

    fn render_info(&self) -> (String, egui::Color32) {
        (self.raw_display(), u8_tup_to_color32(GREY))
    }
}

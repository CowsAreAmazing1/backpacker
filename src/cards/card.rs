use crate::cards::{
    advice::{Advice, AdviceType},
    bonus::Bonus,
    country::Country,
    grey::GreyType,
    special::Special,
};

pub(crate) trait RenderableCard {
    fn render_info(&self) -> (String, egui::Color32);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Card {
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

    pub(crate) fn raw_display(&self) -> String {
        match self {
            Card::Country(country) => country.raw_display(),
            Card::Bonus(bonus) => bonus.raw_display(),
            Card::Advice(advice) => advice.raw_display(),
            Card::Special(special) => special.raw_display(),
            Card::Grey(ty) => ty.raw_display(),
        }
    }

    pub fn deck() -> Vec<Self> {
        vec![
            // Africa
            Card::Country(Country::new("Mali", 10, "cw")),
            Card::Country(Country::new("Egypt", 10, "cw")),
            Card::Country(Country::new("Kenya", 8, "bcw")),
            Card::Country(Country::new("Morocco", 6, "ct")),
            Card::Country(Country::new("Uganda", 6, "ctw")),
            Card::Country(Country::new("South Africa", 4, "bcw")),
            Card::Country(Country::new("Zimbabwe", 2, "cw")),
            // America
            Card::Country(Country::new("Bolivia", 8, "ctw")),
            Card::Country(Country::new("Brazil", 8, "bcw")),
            Card::Country(Country::new("Peru", 8, "ctw")),
            Card::Country(Country::new("Mexico", 6, "bc")),
            Card::Country(Country::new("Argentina", 4, "ctw")),
            Card::Country(Country::new("USA", 2, "bctw")),
            Card::Country(Country::new("Canada", 2, "ctw")),
            // Antarctica
            Card::Country(Country::new("Antarctica", 4, "tw")),
            // Asia
            Card::Country(Country::new("Mongolia", 10, "cw")),
            Card::Country(Country::new("China", 8, "ctw")),
            Card::Country(Country::new("India", 8, "bctw")),
            Card::Country(Country::new("Indonesia", 6, "bctw")),
            Card::Country(Country::new("Nepal", 6, "ctw")),
            Card::Country(Country::new("Uzbekistan", 6, "ct")),
            Card::Country(Country::new("Thailand", 4, "bc")),
            Card::Country(Country::new("Vietnam", 4, "bc")),
            Card::Country(Country::new("Japan", 2, "c")),
            // Europe
            Card::Country(Country::new("Russia", 6, "ctw")),
            Card::Country(Country::new("Turkey", 6, "bc")),
            Card::Country(Country::new("Italy", 4, "ct")),
            Card::Country(Country::new("Germany", 2, "c")),
            Card::Country(Country::new("Ireland", 2, "c")),
            Card::Country(Country::new("UK", 2, "c")),
            Card::Country(Country::new("France", 2, "ct")),
            Card::Country(Country::new("Holland", 2, "c")),
            // Oceania
            Card::Country(Country::new("Easter Island", 6, "c")),
            Card::Country(Country::new("Tahiti", 4, "bcw")),
            Card::Country(Country::new("New Zealand", 4, "bct")),
            Card::Country(Country::new("Australia", 4, "bcw")),
            Card::Country(Country::new("Cook Islands", 2, "bc")),
            Card::Country(Country::new("Fiji", 2, "bcw")),
            // Bonus
            Card::Bonus(Bonus::Beach),
            Card::Bonus(Bonus::Beach),
            Card::Bonus(Bonus::Beach),
            Card::Bonus(Bonus::Culture),
            Card::Bonus(Bonus::Culture),
            Card::Bonus(Bonus::Culture),
            Card::Bonus(Bonus::Trekking),
            Card::Bonus(Bonus::Trekking),
            Card::Bonus(Bonus::Wildlife),
            Card::Bonus(Bonus::Wildlife),
            // Advice
            Card::Advice(Advice::new(true, AdviceType::Money)),
            Card::Advice(Advice::new(true, AdviceType::Money)),
            Card::Advice(Advice::new(true, AdviceType::Money)),
            Card::Advice(Advice::new(true, AdviceType::Transport)),
            Card::Advice(Advice::new(true, AdviceType::Transport)),
            Card::Advice(Advice::new(true, AdviceType::Transport)),
            Card::Advice(Advice::new(false, AdviceType::Transport)),
            Card::Advice(Advice::new(false, AdviceType::Transport)),
            Card::Advice(Advice::new(true, AdviceType::Timing)),
            Card::Advice(Advice::new(true, AdviceType::Timing)),
            Card::Advice(Advice::new(true, AdviceType::Timing)),
            Card::Advice(Advice::new(false, AdviceType::Timing)),
            Card::Advice(Advice::new(false, AdviceType::Timing)),
            Card::Advice(Advice::new(true, AdviceType::Bureaucracy)),
            Card::Advice(Advice::new(true, AdviceType::Bureaucracy)),
            Card::Advice(Advice::new(true, AdviceType::Bureaucracy)),
            Card::Advice(Advice::new(false, AdviceType::Bureaucracy)),
            Card::Advice(Advice::new(false, AdviceType::Bureaucracy)),
            // Grey
            Card::Grey(GreyType::MissedFlight),
            Card::Grey(GreyType::MissedFlight),
            Card::Grey(GreyType::MissedFlight),
            Card::Grey(GreyType::MissedFlight),
            Card::Grey(GreyType::MissedFlight),
            //Special
            Card::Special(Special::CerditCard),
        ]
    }

    pub(crate) fn render_info(&self) -> (String, egui::Color32) {
        match self {
            Card::Country(country) => country.render_info(),
            Card::Bonus(bonus) => bonus.render_info(),
            Card::Advice(advice) => advice.render_info(),
            Card::Special(special) => special.render_info(),
            Card::Grey(ty) => ty.render_info(),
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Country(country) => country.fmt(f),
            Card::Bonus(bonus) => bonus.fmt(f),
            Card::Advice(advice) => advice.fmt(f),
            Card::Special(special) => special.fmt(f),
            Card::Grey(ty) => ty.fmt(f),
        }
    }
}

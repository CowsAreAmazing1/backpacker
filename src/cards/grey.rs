use colored::{Colorize, CustomColor};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum GreyType {
    MissedFlight,
}

impl std::fmt::Display for GreyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grey = CustomColor::new(120, 120, 120);

        match self {
            GreyType::MissedFlight => write!(f, "{}", "Missed Flight".custom_color(grey)),
        }
    }
}

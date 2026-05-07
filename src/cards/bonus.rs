use colored::{Colorize, CustomColor};

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub(crate) enum Bonus {
    Beach,
    Culture,
    Trekking,
    Wildlife,
}

impl Bonus {
    pub(crate) fn parse(input: &char) -> Self {
        match input {
            'b' => Self::Beach,
            'c' => Self::Culture,
            't' => Self::Trekking,
            'w' => Self::Wildlife,
            _ => panic!("Invalid bonus char -> {}", input),
        }
    }

    pub(crate) fn unparse(&self) -> char {
        match self {
            Self::Beach => 'b',
            Self::Culture => 'c',
            Self::Trekking => 't',
            Self::Wildlife => 'w',
        }
    }
}

impl std::fmt::Display for Bonus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = CustomColor::new(106, 229, 218);

        match self {
            Bonus::Beach => write!(f, "{}", "Beach Bonus".custom_color(color)),
            Bonus::Culture => write!(f, "{}", "Culture Bonus".custom_color(color)),
            Bonus::Trekking => write!(f, "{}", "Trekking Bonus".custom_color(color)),
            Bonus::Wildlife => write!(f, "{}", "Wildlife Bonus".custom_color(color)),
        }
    }
}

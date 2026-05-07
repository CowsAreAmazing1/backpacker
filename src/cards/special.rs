use colored::{Colorize, CustomColor};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Special {
    CerditCard,
}

impl std::fmt::Display for Special {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CerditCard => write!(
                f,
                "{}",
                "Credit Card".custom_color(CustomColor::new(231, 157, 72))
            ),
        }
    }
}

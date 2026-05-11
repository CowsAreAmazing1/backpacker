use colored::Colorize;

const SPECIAL: (u8, u8, u8) = (231, 157, 72);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Special {
    CreditCard,
}

impl Special {
    pub(crate) fn raw_display(&self) -> String {
        match self {
            Self::CreditCard => "Credit Card".to_string(),
        }
    }

    pub(crate) fn render_info(&self) -> (String, egui::Color32) {
        (
            self.raw_display(),
            egui::Color32::from_rgb(SPECIAL.0, SPECIAL.1, SPECIAL.2),
        )
    }
}

impl std::fmt::Display for Special {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreditCard => write!(f, "{}", "Credit Card".custom_color(SPECIAL)),
        }
    }
}

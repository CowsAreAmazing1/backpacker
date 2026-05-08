use colored::Colorize;

const GREY: (u8, u8, u8) = (120, 120, 120);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum GreyType {
    MissedFlight,
}

impl GreyType {
    pub(crate) fn raw_display(&self) -> String {
        match self {
            GreyType::MissedFlight => "Missed Flight".to_string(),
        }
    }

    pub(crate) fn render_info(&self) -> (String, egui::Color32) {
        (
            self.raw_display(),
            egui::Color32::from_rgb(GREY.0, GREY.1, GREY.2),
        )
    }
}

impl std::fmt::Display for GreyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreyType::MissedFlight => write!(f, "{}", "Missed Flight".custom_color(GREY)),
        }
    }
}

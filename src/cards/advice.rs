use colored::Colorize;

const GOOD_ADVICE: (u8, u8, u8) = (12, 186, 74);
const BAD_ADVICE: (u8, u8, u8) = (248, 30, 88);

/// Advice flavour enum, used for good and bad advice cards.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum AdviceType {
    Money,
    Bureaucracy,
    Timing,
    Transport,
}

/// Advice card struct, describing the flavour and goodness.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Advice {
    good: bool,
    variant: AdviceType,
}

impl Advice {
    pub(crate) fn new(good: bool, variant: AdviceType) -> Self {
        Self { good, variant }
    }

    /// Raw string representation of the `Advice` card.
    pub(crate) fn raw_display(&self) -> String {
        let pre = if self.good { "Good" } else { "Bad" };
        match self.variant {
            AdviceType::Money => "Money Talks".to_string(),
            AdviceType::Bureaucracy => format!("{} Bureaucracy", pre),
            AdviceType::Timing => format!("{} Timing", pre),
            AdviceType::Transport => format!("{} Transport", pre),
        }
    }

    pub(crate) fn render_info(&self) -> (String, egui::Color32) {
        let text = self.raw_display();
        let color = if self.good {
            egui::Color32::from_rgb(GOOD_ADVICE.0, GOOD_ADVICE.1, GOOD_ADVICE.2)
        } else {
            egui::Color32::from_rgb(BAD_ADVICE.0, BAD_ADVICE.1, BAD_ADVICE.2)
        };
        (text, color)
    }
}

impl std::fmt::Display for Advice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.variant == AdviceType::Money {
            return write!(f, "{}", "Money Talks".green());
        }

        let pre = if self.good { "Good" } else { "Bad" };
        let (c1, c2) = if self.good {
            (
                GOOD_ADVICE, // CustomColor::new(12, 186, 74), // -TODO: maybe use different colors for the text Good/Bad and the type text?
                GOOD_ADVICE, // CustomColor::new(81, 255, 143),
            )
        } else {
            (
                BAD_ADVICE, // CustomColor::new(248, 30, 88),
                BAD_ADVICE, // CustomColor::new(248, 73, 119),
            )
        };

        match self.variant {
            AdviceType::Bureaucracy => write!(
                f,
                "{} {}",
                pre.custom_color(c1),
                "Bureaucracy".custom_color(c2)
            ),
            AdviceType::Timing => {
                write!(f, "{} {}", pre.custom_color(c1), "Timing".custom_color(c2))
            }
            AdviceType::Transport => write!(
                f,
                "{} {}",
                pre.custom_color(c1),
                "Transport".custom_color(c2)
            ),
            _ => panic!("Wont happen"),
        }
    }
}

use colored::{Colorize, CustomColor};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum AdviceType {
    Money,
    Bureaucracy,
    Timing,
    Transport,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Advice {
    good: bool,
    variant: AdviceType,
}

impl Advice {
    pub(crate) fn new(good: bool, variant: AdviceType) -> Self {
        Self { good, variant }
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
                CustomColor::new(12, 186, 74),
                CustomColor::new(81, 255, 143),
            )
        } else {
            (
                CustomColor::new(248, 30, 88),
                CustomColor::new(248, 73, 119),
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

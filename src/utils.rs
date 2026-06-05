use std::{error::Error, fmt};

use crate::cards::advice::AdviceType;

#[derive(Debug)]
pub enum BError {
    Custom(String),

    /// Attempted to play a card before choosing whether to go home
    PlayedBeforeHomeChoice,
    /// Attempted to play too many countries of the same continent, (2 without credit card or 3 with credit card)
    SameContinent,
    /// Attempted to play bonus on country not supporting it
    InvalidBonus,
    /// Attempted to play bonus without top country
    NoTopCountryForBonus,
    /// Attempted to play a country or bonus while affected by Bad Advice
    NoPlayBadAdvice(AdviceType),

    /// Attempted to choose to go home after playing a card
    NoHomePlayed,
    /// Attempted to go home with grey card(s)
    NoHomeGrey,
    /// Attempted to go home while affected by Bad Advice
    NoHomeBadAdvice(AdviceType),

    /// Attempted to discard grey card without suffering
    FreeGreyDiscard,
    /// Attempted to take an action other than choosing a target to attack
    MustChooseTarget,
    /// Attempted to play an offensive card on oneself
    NoSelfTargetting,
    /// Attemped to perform an action other than passing a card to the left
    MustPassCard,
}

impl Error for BError {}

impl fmt::Display for BError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(string) => write!(f, "{}", string),

            Self::PlayedBeforeHomeChoice => write!(f, "choose whether to go home first"),
            Self::SameContinent => write!(f, "too many countries of the same continent"),
            Self::InvalidBonus => write!(f, "can't play that bonus on your top country"),
            Self::NoTopCountryForBonus => write!(f, "need a played country to play a bonus"),
            Self::NoPlayBadAdvice(ty) => write!(
                f,
                "can't play that card while affected by Bad Advice: {:?}",
                ty
            ),

            Self::NoHomePlayed => write!(f, "can't go home after starting your turn"),
            Self::NoHomeGrey => write!(f, "can't go home with grey cards in hand"),
            Self::NoHomeBadAdvice(ty) => {
                write!(f, "can't go home while affected by Bad Advice: {:?}", ty)
            }

            Self::FreeGreyDiscard => write!(f, "can't discard grey cards for free"),
            Self::MustChooseTarget => write!(f, "must chose a target to attack"),
            Self::NoSelfTargetting => write!(f, "cant play offensive cards on yourself"),
            Self::MustPassCard => write!(f, "must pass a card to the left"),
        }
    }
}

// Functions for CLI interaction. This is not up to date rn
fn _read_line() -> Result<String, std::io::Error> {
    let mut buffer = String::new();
    let stdin = std::io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    Ok(buffer)
}

pub(crate) fn _get_requested_input<T, F>(message: &str, condition: F) -> T
where
    T: PartialOrd + std::str::FromStr<Err: std::fmt::Debug>,
    F: Fn(&T) -> bool,
{
    let mut output = None;
    while output.is_none() {
        println!("{}", message);
        let inp_opt = text_io::try_read!();

        match inp_opt {
            Ok(inp) => {
                if condition(&inp) {
                    output = Some(inp);
                } else {
                    println!("Invalid value\n");
                }
            }
            Err(_) => println!("Error reading input\n"),
        }
    }

    output.unwrap()
}

pub(crate) fn u8_tup_to_color32(tup: (u8, u8, u8)) -> egui::Color32 {
    egui::Color32::from_rgb(tup.0, tup.1, tup.2)
}

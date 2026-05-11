use std::{error::Error, fmt};

#[derive(Debug)]
pub enum BError {
    Custom(String),
    // Attempted to play too many countries of the same continent, (2 without credit card or 3 with credit card)
    SameContinent,
    // Attempted to go home with grey card(s)
    GreyHeld,
    // Attempted to play bonus on country not supporting it
    InvalidBonus,
    // Attempted to play bonus without top country
    NoTopCountry,
    // Attempted to discard grey card without suffering
    FreeGreyDiscard,
    // Attempted to play a card before choosing whether to go home
    PlayedBeforeHomeChoice,
    // Attempted to choose to go home after playing a card
    HomeChoiceMissed,
}

impl Error for BError {}

impl fmt::Display for BError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom(string) => write!(f, "{}", string),
            Self::SameContinent => write!(f, "too many countries of the same continent"),
            Self::GreyHeld => write!(f, "can't go home with grey cards"),
            Self::InvalidBonus => write!(f, "can't play that bonus on your top country"),
            Self::NoTopCountry => write!(f, "need a played country to play a bonus"),
            Self::FreeGreyDiscard => write!(f, "can't discard grey cards for free"),
            Self::PlayedBeforeHomeChoice => {
                write!(f, "choose whether to go home before playing cards")
            }
            Self::HomeChoiceMissed => {
                write!(f, "can't go home after starting your turn")
            }
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

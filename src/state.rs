use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardView {
    pub label: String,
    pub color: [u8; 4],
    pub is_grey: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CountryView {
    pub label: String,
    pub color: [u8; 4],
    pub bonuses: Vec<CardView>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerView {
    pub hand: Vec<CardView>,
    pub pile: Vec<CountryView>,
    pub score: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameSnapshot {
    pub turn: usize,
    pub turn_stage: String,
    pub future: Vec<CardView>,
    pub past: Vec<CardView>,
    pub players: Vec<PlayerView>,
    /// If a multi-step play is in-progress, index of the card in the active player's hand
    pub pending_play: Option<usize>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ClientMsg {
    Play { index: usize },
    Discard { index: usize },
    GoHome { go: bool },
    ChooseTarget { target: usize },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ServerMsg {
    State { state: GameSnapshot },
    Error { message: String },
}

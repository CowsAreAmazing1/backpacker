// Module declarations
mod board;
mod player;
mod status;
mod ui;
mod utils;

pub mod cards {
    pub mod advice;
    pub mod bonus;
    pub mod card;
    pub mod continent;
    pub mod country;
    pub mod grey;
    pub mod special;
}

pub use ui::GameEgui;

pub const HAND_SIZE: usize = 5;
pub const PAUSE_TIME: u64 = 700;

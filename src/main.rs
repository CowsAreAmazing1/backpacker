use backpacker::{Board};

fn main() {
    let mut board = Board::new_game(1);

    for player in &mut board.players {
        player.try_playing_all_counties();
    }
}

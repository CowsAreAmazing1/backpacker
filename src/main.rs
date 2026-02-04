use backpacker::{Board, StatusType};

fn main() {
    let mut board = Board::new_game(1);

    board.players[0].add_status(StatusType::MissGo(10));

    board.manual_game();
}

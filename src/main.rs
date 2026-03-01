use backpacker::Board;

fn main() {
    let mut board = Board::new_game(2);
    board.manual_game();
}

// let to_have = [
//     |cards: &Vec<backpacker::Card>| cards.iter().any(|card| matches!(card, backpacker::Card::Country(backpacker::Country { name, ..}) if *name == "Mali")),
//     |cards: &Vec<backpacker::Card>| cards.iter().any(|card| matches!(card, backpacker::Card::Country(backpacker::Country { name, ..}) if *name == "Kenya")),
//     |cards: &Vec<backpacker::Card>| cards.iter().any(|card| matches!(card, backpacker::Card::Special(backpacker::Special::CerditCard))),
// ];

// let mut board;
// loop {
//     board = Board::new_game(2);
//     let cards = &board.players[0].hand;
//     if to_have.iter().all(|func| func(cards)) {
//         break;
//     }
// }
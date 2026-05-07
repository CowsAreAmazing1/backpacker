use backpacker::GameEgui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "grahpher",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(GameEgui::new(cc)))),
    )
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

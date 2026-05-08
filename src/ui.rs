use crate::board::Board;

pub struct GameEgui {
    board: Board,
}

impl GameEgui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let board = Board::new_game(2);

        Self { board }
    }
}

impl eframe::App for GameEgui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.board.turn_heading();
        self.board.manual_turn();

        // egui::CentralPanel::default().show(ctx, |ui| {})

        for (i, player) in self.board.players.iter().enumerate() {
            egui::Window::new(format!("Player {}", i)).show(ctx, |ui| {
                ui.heading(format!("Player {}", i + 1));

                ui.label("Hand:");

                for (i, card) in player.iter_hand().enumerate() {
                    ui.label(format!("   - {} {}", i + 1, card.raw_display()));
                }

                // All player's played piles
                for card in player.iter_pile() {
                    let bonus_text = player
                        .top_country()
                        .map_or_else(|| "".to_string(), |country| country.raw_display())
                        .to_uppercase();

                    println!("| {} - {}", card, bonus_text);
                    for bonus in card.bonus.iter() {
                        println!("| ↳ {}", bonus)
                    }
                }
            });
        }
    }
}

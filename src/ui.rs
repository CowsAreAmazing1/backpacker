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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // egui::CentralPanel::default().show(ctx, |ui| {})

        for (i, player) in self.board.players.iter().enumerate() {
            egui::Window::new(format!("Player {}", i))
                .show(ctx, |ui| ui.add(egui::Label::new("hello im a player")));
        }
    }
}

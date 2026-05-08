use crate::{
    board::{Board, BoardMove},
    cards::card::RenderableCard,
};

pub struct GameEgui {
    board: Board,
    status_message: Option<String>,
}

impl GameEgui {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let board = Board::new_game(2);

        Self {
            board,
            status_message: None,
        }
    }
}

fn draw_card<C: RenderableCard>(
    ui: &mut egui::Ui,
    card: &C,
    prefix: String,
    enabled: bool,
) -> bool {
    let mut clicked = false;
    let (text, color) = card.render_info();
    let button = egui::Button::new("").fill(color).small();

    ui.add_enabled_ui(enabled, |ui| {
        ui.horizontal(|ui| {
            ui.label(prefix);
            clicked = ui.add(button).clicked();
            ui.label(text.to_string());
        });
    });

    clicked
}

impl eframe::App for GameEgui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // self.board.turn_heading();
        // self.board.manual_turn();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(format!("Player {}'s turn", self.board.current_turn() + 1));
            if let Some(message) = &self.status_message {
                ui.label(message);
            }

            // Player info section
            egui::Grid::new("some id").show(ui, |ui| {
                // Player `i` headings
                for i in 0..self.board.players.len() {
                    ui.centered_and_justified(|ui| ui.heading(format!("Player {}", i + 1)));
                }
                ui.end_row();

                // Hands, shown in vertical groups, listing each card and a button
                for player_idx in 0..self.board.players.len() {
                    let is_active_player = player_idx == self.board.current_turn();
                    let mut played_card_idx = None;

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            for (card_idx, card) in
                                self.board.players[player_idx].iter_hand().enumerate()
                            {
                                // Draw the card. If it was clicked, set `played_card_idx` to the index of the card in the player's hand, so it can be sent to the board to be played.
                                draw_card(ui, card, (card_idx + 1).to_string(), is_active_player)
                                    .then(|| played_card_idx = Some(card_idx));
                            }
                        })
                    });

                    if let Some(card_idx) = played_card_idx {
                        match self.board.make_move(BoardMove::PlayCard(card_idx)) {
                            Ok(()) => self.status_message = None,
                            Err(err) => self.status_message = Some(err.to_string()),
                        }
                    }
                }
                ui.end_row();

                for player in self.board.players.iter() {
                    if player.top_country().is_none() {
                        continue;
                    }

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            for (card_idx, card) in player.iter_pile().enumerate() {
                                draw_card(ui, card, (card_idx + 1).to_string(), true);
                                for bonus in card.bonus.iter() {
                                    draw_card(ui, bonus, "⮩".to_string(), true);
                                }
                            }
                        });
                    });
                }
            });
        });
    }
}

use egui::Button;

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

fn draw_card<C: RenderableCard>(ui: &mut egui::Ui, card: &C, prefix: String) -> bool {
    let mut clicked = false;
    let (text, color) = card.render_info();
    let button = Button::new("").fill(color).small();
    ui.horizontal(|ui| {
        ui.label(prefix);
        clicked = ui.add(button).clicked();
        ui.label(text.to_string());
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
                    let hand: Vec<_> = self.board.players[player_idx]
                        .iter_hand()
                        .enumerate()
                        .map(|(card_idx, card)| {
                            let (text, color) = card.render_info();
                            (card_idx, text, color)
                        })
                        .collect();

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            for (card_idx, text, color) in hand {
                                ui.horizontal(|ui| {
                                    if is_active_player {
                                        let button = Button::new("").fill(color).small();
                                        if ui.add(button).clicked() {
                                            match self
                                                .board
                                                .make_move(BoardMove::PlayCard(card_idx))
                                            {
                                                Ok(()) => self.status_message = None,
                                                Err(err) => {
                                                    self.status_message = Some(err.to_string())
                                                }
                                            }
                                        }
                                    } else {
                                        ui.add_enabled(false, Button::new("").fill(color).small());
                                    }
                                    ui.label(format!(" - {} {}", card_idx + 1, text));
                                });
                            }
                        })
                    });
                }
                ui.end_row();

                for player in self.board.players.iter() {
                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            for (card_idx, card) in player.iter_pile().enumerate() {
                                draw_card(ui, card, (card_idx + 1).to_string());
                                // let (text, color) = card.render_info();
                                // ui.horizontal(|ui| {
                                //     ui.add(Button::new("").fill(color).small());
                                //     ui.label(format!(" - {} {}", card_idx + 1, text));
                                // });
                                for bonus in card.bonus.iter() {
                                    draw_card(ui, bonus, "⮩".to_string());
                                }
                            }
                        });
                    });
                }
            });

            // egui::Grid::new("some_unique_id").show(ui, |ui| {
            //     ui.label("First row, first column");
            //     ui.group(|ui| {
            //         for (i, player) in self.board.players.iter().enumerate() {
            //             ui.label(format!("Player {}", i));
            //             // egui::Window::new(format!("Player {}", i + 1))
            //             //     .collapsible(false)
            //             //     .show(ctx, |ui| {
            //             //         ui.label("Hand:");

            //             //         for (i, card) in player.iter_hand().enumerate() {
            //             //             let (text, color) = card.render_info();
            //             //             ui.horizontal(|ui| {
            //             //                 let button = Button::new("").fill(color).small();
            //             //                 if ui.add(button).clicked() {
            //             //                     println!("clicked on {}", i)
            //             //                 }
            //             //                 ui.label(format!("   - {} {}", i + 1, text));
            //             //             });
            //             //         }

            //             //         // // All player's played piles
            //             //         // for card in player.iter_pile() {
            //             //         //     let bonus_text = player
            //             //         //         .top_country()
            //             //         //         .map_or_else(|| "".to_string(), |country| country.raw_display())
            //             //         //         .to_uppercase();

            //             //         //     println!("| {} - {}", card, bonus_text);
            //             //         //     for bonus in card.bonus.iter() {
            //             //         //         println!("| ↳ {}", bonus)
            //             //         //     }
            //             //         // }
            //             //     });
            //             // // ui.memory_mut(|mem| mem.reset_areas());
            //         }
            //     });

            //     ui.end_row();

            //     ui.horizontal(|ui| {
            //         ui.label("Same");
            //         ui.label("cell");
            //     });
            //     ui.label("Third row, second column");
            //     ui.end_row();
            // });
        });
    }
}

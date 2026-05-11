use egui::{Color32, RichText};
use itertools::{EitherOrBoth, Itertools};

use crate::{
    board::{Board, PlayerAction, TurnStage},
    cards::card::RenderableCard,
};

#[cfg(target_arch = "wasm32")]
use crate::state::{CardView, ClientMsg, GameSnapshot, ServerMsg};

#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, JsValue, closure::Closure};

#[cfg(target_arch = "wasm32")]
use web_sys::{Event, MessageEvent, WebSocket};

pub struct GameEgui {
    backend: GameBackend,
}

enum GameBackend {
    Local {
        board: Board,
        status_message: Option<String>,
    },
    #[cfg(target_arch = "wasm32")]
    Remote(RemoteGameClient),
}

impl GameEgui {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            backend: GameBackend::Local {
                board: Board::new_game(),
                status_message: None,
            },
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            backend: GameBackend::Remote(RemoteGameClient::new(&cc.egui_ctx)),
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

#[cfg(target_arch = "wasm32")]
fn draw_card_view(ui: &mut egui::Ui, card: &CardView, prefix: String, enabled: bool) -> bool {
    let color = Color32::from_rgba_premultiplied(
        card.color[0],
        card.color[1],
        card.color[2],
        card.color[3],
    );
    let button = egui::Button::new("").fill(color).small();
    let mut clicked = false;

    ui.add_enabled_ui(enabled, |ui| {
        ui.horizontal(|ui| {
            ui.label(prefix);
            clicked = ui.add(button).clicked();
            ui.label(card.label.clone());
        });
    });

    clicked
}

impl eframe::App for GameEgui {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
            ui.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        match &mut self.backend {
            GameBackend::Local {
                board,
                status_message,
            } => {
                render_local_board(ui, board, status_message);
            }
            #[cfg(target_arch = "wasm32")]
            GameBackend::Remote(client) => {
                render_remote_board(ui, client);
            }
        }
    }
}

fn render_local_board(ui: &mut egui::Ui, board: &mut Board, status_message: &mut Option<String>) {
    egui::Panel::left("game info").show_inside(ui, |ui| {
        ui.heading(format!("Player {}'s turn", board.turn + 1));
        ui.label(board.turn_stage.to_string());
        ui.separator();
        if let Some(message) = status_message.as_ref() {
            ui.label(message);
        }

        ui.separator();

        egui::Grid::new("card storage").show(ui, |ui| {
            ui.label("Future".to_string());
            ui.label("Past".to_string());
            ui.end_row();

            for pair in board.future.iter().rev().zip_longest(board.past.iter()) {
                match pair {
                    EitherOrBoth::Both(future_card, past_card) => {
                        draw_card(ui, future_card, "".to_string(), false);
                        draw_card(ui, past_card, "".to_string(), false);
                    }
                    EitherOrBoth::Left(future_card) => {
                        draw_card(ui, future_card, "".to_string(), false);
                    }
                    EitherOrBoth::Right(past_card) => {
                        draw_card(ui, past_card, "".to_string(), false);
                    }
                };
                ui.end_row();
            }
        })
    });

    egui::CentralPanel::default().show_inside(ui, |ui| {
        egui::Grid::new("game info").show(ui, |ui| {
            for (i, player) in board.players.iter().enumerate() {
                ui.group(|ui| {
                    ui.centered_and_justified(|ui| {
                        ui.horizontal(|ui| {
                            ui.heading(format!("Player {}", i + 1));
                            ui.label(player.score.to_string());
                        });
                    })
                });
            }
            ui.end_row();

            if board.turn_stage == TurnStage::ChooseGoHome {
                for player_idx in 0..board.players.len() {
                    ui.centered_and_justified(|ui| {
                        if player_idx == board.turn {
                            let color = Color32::from_rgb(0, 55, 79);
                            let t = (2.0 * ui.time()).sin() * 0.5 + 0.5;
                            let faded = color.lerp_to_gamma(color.gamma_multiply(0.2), t as f32);

                            ui.request_repaint();

                            egui::Frame::group(ui.style()).fill(faded).show(ui, |ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Go Home?: ");
                                    let result = if ui
                                        .button(RichText::new("✔").color(Color32::GREEN))
                                        .clicked()
                                    {
                                        board.apply_action(PlayerAction::GoHome(true))
                                    } else if ui
                                        .button(RichText::new("✖").color(Color32::RED))
                                        .clicked()
                                    {
                                        board.apply_action(PlayerAction::GoHome(false))
                                    } else {
                                        Ok(())
                                    };

                                    if let Err(err) = result {
                                        *status_message = Some(err.to_string())
                                    }
                                });
                            });
                        } else if player_idx < board.turn {
                            ui.label("➡");
                        } else {
                            ui.label("⬅");
                        }
                    });
                }
                ui.end_row();
            }

            for player_idx in 0..board.players.len() {
                let is_active_player = player_idx == board.turn;
                let mut played_card_idx = None;
                let mut discarded_card_idx = None;

                ui.group(|ui| {
                    ui.vertical(|ui| {
                        for (card_idx, card) in board.players[player_idx].iter_hand().enumerate() {
                            ui.horizontal(|ui| {
                                draw_card(ui, card, (card_idx + 1).to_string(), is_active_player)
                                    .then(|| played_card_idx = Some(card_idx));
                                if !card.is_grey()
                                    && is_active_player
                                    && board.turn_stage != TurnStage::ChooseGoHome
                                    && ui.button("🕯").clicked()
                                {
                                    discarded_card_idx = Some(card_idx);
                                }
                            });
                        }
                    })
                });

                if let Some(card_idx) = played_card_idx {
                    match board.apply_action(PlayerAction::Play(card_idx)) {
                        Ok(()) => *status_message = None,
                        Err(err) => *status_message = Some(err.to_string()),
                    }
                }
                if let Some(card_idx) = discarded_card_idx {
                    match board.apply_action(PlayerAction::Discard(card_idx)) {
                        Ok(()) => *status_message = None,
                        Err(err) => *status_message = Some(err.to_string()),
                    }
                }
            }
            ui.end_row();

            for player in board.players.iter() {
                if player.top_country().is_none() {
                    ui.label("");
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

#[cfg(target_arch = "wasm32")]
fn render_remote_board(ui: &mut egui::Ui, client: &RemoteGameClient) {
    let snapshot = client.snapshot();
    let status_message = client.status_message();

    if let Some(snapshot) = snapshot {
        egui::Panel::left("game info").show_inside(ui, |ui| {
            ui.heading(format!("Player {}'s turn", snapshot.turn + 1));
            ui.label(snapshot.turn_stage.clone());
            ui.separator();
            if let Some(message) = &status_message {
                ui.label(message);
            }

            ui.separator();

            egui::Grid::new("card storage").show(ui, |ui| {
                ui.label("Future".to_string());
                ui.label("Past".to_string());
                ui.end_row();

                for pair in snapshot
                    .future
                    .iter()
                    .rev()
                    .zip_longest(snapshot.past.iter())
                {
                    match pair {
                        EitherOrBoth::Both(future_card, past_card) => {
                            draw_card_view(ui, future_card, "".to_string(), false);
                            draw_card_view(ui, past_card, "".to_string(), false);
                        }
                        EitherOrBoth::Left(future_card) => {
                            draw_card_view(ui, future_card, "".to_string(), false);
                        }
                        EitherOrBoth::Right(past_card) => {
                            draw_card_view(ui, past_card, "".to_string(), false);
                        }
                    };
                    ui.end_row();
                }
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::Grid::new("game info").show(ui, |ui| {
                for (i, player) in snapshot.players.iter().enumerate() {
                    ui.group(|ui| {
                        ui.heading(format!("Player {}", i + 1));
                        ui.label(player.score.to_string());
                    });
                }
                ui.end_row();

                if snapshot.turn_stage == "ChooseGoHome" {
                    for player_idx in 0..snapshot.players.len() {
                        ui.centered_and_justified(|ui| {
                            if player_idx == snapshot.turn {
                                let color = Color32::from_rgb(0, 55, 79);
                                let t = (2.0 * ui.time()).sin() * 0.5 + 0.5;
                                let faded =
                                    color.lerp_to_gamma(color.gamma_multiply(0.2), t as f32);

                                ui.request_repaint();

                                egui::Frame::group(ui.style()).fill(faded).show(ui, |ui| {
                                    ui.horizontal(|ui| {
                                        ui.label("Go Home?: ");
                                        if ui
                                            .button(RichText::new("✔").color(Color32::GREEN))
                                            .clicked()
                                        {
                                            client.send(ClientMsg::GoHome { go: true });
                                        } else if ui
                                            .button(RichText::new("✖").color(Color32::RED))
                                            .clicked()
                                        {
                                            client.send(ClientMsg::GoHome { go: false });
                                        }
                                    });
                                });
                            } else if player_idx < snapshot.turn {
                                ui.label("➡");
                            } else {
                                ui.label("⬅");
                            }
                        });
                    }
                }
                ui.end_row();

                for player_idx in 0..snapshot.players.len() {
                    let is_active_player = player_idx == snapshot.turn;
                    let mut played_card_idx = None;
                    let mut discarded_card_idx = None;

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            for (card_idx, card) in
                                snapshot.players[player_idx].hand.iter().enumerate()
                            {
                                ui.horizontal(|ui| {
                                    draw_card_view(
                                        ui,
                                        card,
                                        (card_idx + 1).to_string(),
                                        is_active_player,
                                    )
                                    .then(|| played_card_idx = Some(card_idx));
                                    if !card.is_grey
                                        && is_active_player
                                        && snapshot.turn_stage != "ChooseGoHome"
                                        && ui.button("🕯").clicked()
                                    {
                                        discarded_card_idx = Some(card_idx);
                                    }
                                });
                            }
                        })
                    });

                    if let Some(card_idx) = played_card_idx {
                        client.send(ClientMsg::Play { index: card_idx });
                    }
                    if let Some(card_idx) = discarded_card_idx {
                        client.send(ClientMsg::Discard { index: card_idx });
                    }
                }
                ui.end_row();

                for player in snapshot.players.iter() {
                    if player.pile.is_empty() {
                        ui.label("");
                        continue;
                    }

                    ui.group(|ui| {
                        ui.vertical(|ui| {
                            for (card_idx, card) in player.pile.iter().enumerate() {
                                draw_card_view(
                                    ui,
                                    &CardView {
                                        label: card.label.clone(),
                                        color: card.color,
                                        is_grey: false,
                                    },
                                    (card_idx + 1).to_string(),
                                    true,
                                );
                                for bonus in card.bonuses.iter() {
                                    draw_card_view(ui, bonus, "⮩".to_string(), true);
                                }
                            }
                        });
                    });
                }
            });
        });
    } else {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Connecting to the shared game...");
                if let Some(message) = status_message {
                    ui.label(message);
                }
            });
        });
    }
}

#[cfg(target_arch = "wasm32")]
struct RemoteGameClient {
    state: Rc<RefCell<RemoteState>>,
    ws: WebSocket,
    _on_message: Closure<dyn FnMut(MessageEvent)>,
    _on_open: Closure<dyn FnMut(Event)>,
    _on_error: Closure<dyn FnMut(web_sys::ErrorEvent)>,
}

#[cfg(target_arch = "wasm32")]
struct RemoteState {
    snapshot: Option<GameSnapshot>,
    status_message: Option<String>,
}

#[cfg(target_arch = "wasm32")]
impl RemoteGameClient {
    fn new(ctx: &egui::Context) -> Self {
        let ws = WebSocket::new(&websocket_url()).expect("failed to connect to websocket");
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let state = Rc::new(RefCell::new(RemoteState {
            snapshot: None,
            status_message: Some("Connecting to shared game...".to_string()),
        }));
        let repaint_ctx = ctx.clone();

        let on_message_state = state.clone();
        let on_message_ctx = repaint_ctx.clone();
        let on_message = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Some(text) = event.data().as_string() {
                web_sys::console::log_1(&JsValue::from_str("[backpacker] ws message received"));
                match serde_json::from_str::<ServerMsg>(&text) {
                    Ok(ServerMsg::State { state: snapshot }) => {
                        let mut state = on_message_state.borrow_mut();
                        state.snapshot = Some(snapshot);
                        state.status_message = None;
                        on_message_ctx.request_repaint();
                    }
                    Ok(ServerMsg::Error { message }) => {
                        let mut state = on_message_state.borrow_mut();
                        state.status_message = Some(message);
                        on_message_ctx.request_repaint();
                    }
                    Err(err) => {
                        let mut state = on_message_state.borrow_mut();
                        state.status_message = Some(format!("Bad server message: {err}"));
                        on_message_ctx.request_repaint();
                    }
                }
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        ws.set_onmessage(Some(on_message.as_ref().unchecked_ref()));

        let on_open_state = state.clone();
        let on_open_ctx = repaint_ctx.clone();
        let on_open = Closure::wrap(Box::new(move |_event: Event| {
            web_sys::console::log_1(&JsValue::from_str("[backpacker] ws connected"));
            on_open_state.borrow_mut().status_message = Some("Connected".to_string());
            on_open_ctx.request_repaint();
        }) as Box<dyn FnMut(Event)>);
        ws.set_onopen(Some(on_open.as_ref().unchecked_ref()));

        let on_error_state = state.clone();
        let on_error_ctx = repaint_ctx;
        let on_error = Closure::wrap(Box::new(move |_event: web_sys::ErrorEvent| {
            web_sys::console::error_1(&JsValue::from_str("[backpacker] ws error"));
            on_error_state.borrow_mut().status_message = Some("WebSocket error".to_string());
            on_error_ctx.request_repaint();
        }) as Box<dyn FnMut(web_sys::ErrorEvent)>);
        ws.set_onerror(Some(on_error.as_ref().unchecked_ref()));

        Self {
            state,
            ws,
            _on_message: on_message,
            _on_open: on_open,
            _on_error: on_error,
        }
    }

    fn snapshot(&self) -> Option<GameSnapshot> {
        self.state.borrow().snapshot.clone()
    }

    fn status_message(&self) -> Option<String> {
        self.state.borrow().status_message.clone()
    }

    fn send(&self, msg: ClientMsg) {
        if let Ok(text) = serde_json::to_string(&msg) {
            let _ = self.ws.send_with_str(&text);
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn websocket_url() -> String {
    let window = web_sys::window().expect("window unavailable");
    let location = window.location();
    let protocol = location.protocol().unwrap_or_else(|_| "http:".to_string());

    if protocol == "file:" {
        return "ws://127.0.0.1:3030/ws".to_string();
    }

    let host = location
        .host()
        .unwrap_or_else(|_| "127.0.0.1:3030".to_string());
    let scheme = if protocol == "https:" { "wss" } else { "ws" };
    let url = format!("{scheme}://{host}/ws");
    web_sys::console::log_1(&JsValue::from_str(&format!("[backpacker] ws url: {url}")));
    url
}

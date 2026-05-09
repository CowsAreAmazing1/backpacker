#[cfg(not(target_arch = "wasm32"))]
use std::{net::SocketAddr, sync::Arc, thread, time::Duration};

#[cfg(not(target_arch = "wasm32"))]
use futures::SinkExt;
#[cfg(not(target_arch = "wasm32"))]
use futures::{FutureExt, StreamExt};
#[cfg(not(target_arch = "wasm32"))]
use tokio::sync::{Mutex, broadcast};
#[cfg(not(target_arch = "wasm32"))]
use warp::Filter;
#[cfg(not(target_arch = "wasm32"))]
use warp::ws::{Message, WebSocket};

#[cfg(not(target_arch = "wasm32"))]
use backpacker::board::Board;
#[cfg(not(target_arch = "wasm32"))]
use backpacker::board::PlayerAction;
#[cfg(not(target_arch = "wasm32"))]
use backpacker::state::{ClientMsg, ServerMsg};

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    let board = Board::new_game();
    let board = Arc::new(Mutex::new(board));

    // broadcast channel for server->clients
    let (tx, _rx) = broadcast::channel::<String>(16);
    let tx_filter = warp::any().map(move || tx.clone());
    let board_filter = warp::any().map(move || board.clone());

    // WebSocket route
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(tx_filter)
        .and(board_filter)
        .map(|ws: warp::ws::Ws, tx, board| {
            ws.on_upgrade(move |socket| client_connected(socket, tx, board))
        });

    // serve static files from current directory
    let static_files = warp::fs::dir(".");

    let routes = ws_route
        .or(static_files)
        .with(warp::cors().allow_any_origin());

    let addr: SocketAddr = ([0, 0, 0, 0], 3030).into();
    println!("Serving on http://0.0.0.0:3030 (local: http://127.0.0.1:3030)");
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(600));
        let _ = webbrowser::open("http://127.0.0.1:3030/");
    });
    warp::serve(routes).run(addr).await;
}

#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(not(target_arch = "wasm32"))]
async fn client_connected(ws: WebSocket, tx: broadcast::Sender<String>, board: Arc<Mutex<Board>>) {
    let (mut ws_tx, mut ws_rx) = ws.split();

    // send initial snapshot
    let b = board.lock().await;
    let snap = b.snapshot();
    let msg = serde_json::to_string(&ServerMsg::State { state: snap }).unwrap();
    let _ = ws_tx.send(Message::text(msg)).await;
    drop(b);

    // subscribe to broadcast channel for server-wide updates
    let mut rx = tx.subscribe();

    // task to forward broadcast messages to websocket
    let send_task = async move {
        while let Ok(msg) = rx.recv().await {
            if ws_tx.send(Message::text(msg)).await.is_err() {
                break;
            }
        }
    }
    .fuse();

    // task to receive client messages
    let recv_task = async move {
        while let Some(Ok(msg)) = ws_rx.next().await {
            if msg.is_text()
                && let Ok(text) = msg.to_str()
            {
                match serde_json::from_str::<ClientMsg>(text) {
                    Ok(client_msg) => {
                        handle_client_msg(client_msg, board.clone(), tx.clone()).await;
                    }
                    Err(err) => {
                        eprintln!("Invalid client message: {}", err);
                    }
                }
            }
        }
    }
    .fuse();

    futures::pin_mut!(send_task, recv_task);
    futures::select! {
        _ = send_task => (),
        _ = recv_task => (),
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn handle_client_msg(
    msg: ClientMsg,
    board: Arc<Mutex<Board>>,
    tx: broadcast::Sender<String>,
) {
    let mut b = board.lock().await;
    let res = match msg {
        ClientMsg::Play { index } => b.apply_action(PlayerAction::Play(index)),
        ClientMsg::Discard { index } => b.apply_action(PlayerAction::Discard(index)),
        ClientMsg::GoHome { go } => b.apply_action(PlayerAction::GoHome(go)),
    };

    match res {
        Ok(()) => {
            let snap = b.snapshot();
            let msg_text = serde_json::to_string(&ServerMsg::State { state: snap }).unwrap();
            let _ = tx.send(msg_text);
        }
        Err(e) => {
            let msg_text = serde_json::to_string(&ServerMsg::Error {
                message: format!("{:?}", e),
            })
            .unwrap();
            let _ = tx.send(msg_text);
        }
    }
}

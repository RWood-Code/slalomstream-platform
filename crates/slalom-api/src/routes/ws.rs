use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures_util::{SinkExt, StreamExt};

use crate::state::AppState;

pub async fn ws_handler(ws: WebSocketUpgrade, State(st): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle(socket, st))
}

async fn handle(socket: WebSocket, st: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut bus_rx = st.bus.subscribe();

    let send_task = tokio::spawn(async move {
        while let Ok(msg) = bus_rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    while let Some(Ok(msg)) = receiver.next().await {
        if matches!(msg, Message::Close(_)) {
            break;
        }
    }
    send_task.abort();
}

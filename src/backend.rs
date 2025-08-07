use std::{collections::HashMap, sync::Arc, time::Duration};

use axum::extract::ws::{Message, WebSocket};
use dioxus::html::{data, ms, script::r#async};
use futures::StreamExt;
use serde_json::{to_string, Value};
use tokio::{
    sync::{broadcast, mpsc, Mutex},
    time::interval,
};
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    connections: Arc<Mutex<HashMap<String, broadcast::Sender<Message>>>>,
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let conn_id = Uuid::new_v4().to_string();
    let conn_id_clone = conn_id.clone();

    let (tx, mut rx) = broadcast::channel(100);
    {
        let mut connections = state.connections.lock().await;
        connections.insert(conn_id.clone(), tx.clone());
    }

    let (mut sender, mut reciever) = socket.split();
    let (message_tx, mut message_rx) = mpsc::channel::<Message>(100);

    let sender_task = tokio::spawn(async move {
        while let some(msg) = message_rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    let ping_tx = message_tx.clone();
    let ping_task = tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(30));
        loop {
            interval.tick().await;
            if ping_tx.send(Message::Ping(vec![])).await.is_err() {
                break;
            }
        }
    });

    let forward_tx = message_tx.clone();
    let forward_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if forward_tx.send(msg).await.is_err() {
                break;
            }
        }
    });

    let receive_task = tokio::spawn({
        let state = state.clone();
        let tx = tx.clone();
        let mut target_map: HashMap<String, String> = HashMap::new();

        async move {
            while let Some(Ok(msg)) = reciever.next().await {
                match msg {
                    Message::Text(text) => {
                        if let Ok(data) = serde_json::from_str::<Value>(&text) {
                            if data["type"] == "register" {
                                if let Some(id) = data["connectionId"].as_str() {
                                    state
                                        .connections
                                        .lock()
                                        .await
                                        .insert(id.to_string(), tx.clone());
                                }
                                continue;
                            }

                            if let Some(target_id) = data["target_id"].as_str() {
                                target_map.insert(conn_id.clone(), target_id.to_string());
                                if let Some(target_tx) =
                                    state.connections.lock().await.get(target_id)
                                {
                                    let _ = target_tx.send(Message::Text((text)));
                                }
                            }
                        }
                    }
                    Message::Binary(bin_data) => {
                        if let Some(target_id) = target_map.get(&conn_id) {}
                    }
                }
            }
        }
    });
}

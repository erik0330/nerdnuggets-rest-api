use axum::{
    extract::{
        ws::{Message, WebSocket},
        Query, WebSocketUpgrade,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use database::{AppService, DatabasePool};
use futures_util::{SinkExt, StreamExt};
use index_store::DummyStore;
use notification_websockets::{run_notifications_pusher, UserSockets};
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;
use tracing::info;
use types::error::Error;
use utils::env::Env;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let env = Env::init();
    tracing_subscriber::fmt::init();

    info!("Start initializing notification websockes");

    let args: Vec<String> = std::env::args().collect();
    let index = args.get(1).map(|a| a.parse::<i64>().unwrap());
    let index_store = DummyStore::new(index);

    let connection = DatabasePool::init(&env)
        .await
        .unwrap_or_else(|e| panic!("Database error: {e}"));
    let db = Arc::new(connection);
    let service = AppService::init(&db, &env);

    let user_sockets = Arc::new(Mutex::new(HashMap::new()));

    run_notifications_pusher(service, index_store, 1, Arc::clone(&user_sockets));

    info!("Finish initialization complete");

    let app = Router::new().route(
        "/ws",
        get(move |ws, query| websocket_handler(ws, query, Arc::clone(&user_sockets))),
    );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await.unwrap();
    info!("WebSocket server running on ws://0.0.0.0:8001/ws?userId=USER_ID");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    Query(params): Query<HashMap<String, String>>,
    state: UserSockets,
) -> impl IntoResponse {
    if let Some(user_id) = params
        .get("userId")
        .cloned()
        .and_then(|u| Uuid::from_str(&u).ok())
    {
        ws.on_upgrade(move |socket| handle_socket(socket, user_id, state))
    } else {
        "Missing userId query parameter".into_response()
    }
}

async fn handle_socket(socket: WebSocket, user_id: Uuid, state: UserSockets) {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let (mut sender, mut receiver) = socket.split();

    // Clone user_id upfront for use in async tasks and cleanup
    let user_id_task = user_id.clone();

    // Store the new connection
    {
        let mut user_sockets = state.lock().unwrap();
        user_sockets
            .entry(user_id.clone())
            .or_insert(Vec::new())
            .push(tx);
    }

    // Task to receive messages from the WebSocket
    tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(_) => {}
                Message::Binary(_) => {}
                Message::Ping(_) => {}
                Message::Pong(_) => {}
                Message::Close(_) => {
                    break;
                }
            }
        }
        // Cleanup when user disconnects
        let mut user_sockets = state.lock().unwrap();
        if let Some(sockets) = user_sockets.get_mut(&user_id_task) {
            sockets.retain(|s| !s.is_closed() && s.send(Message::Ping(vec![])).is_ok());
            if sockets.is_empty() {
                user_sockets.remove(&user_id_task);
            }
        }
    });

    // Task to send messages to the WebSocket
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break; // If sending fails, exit the loop
            }
        }
    });
}

use crate::pusher::Pusher;
use crate::reader::Reader;
use axum::extract::ws::Message;
use database::AppService;
use index_store::IndexStore;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;
use tracing::info;
use uuid::Uuid;

mod pusher;
mod reader;

type Sender = mpsc::UnboundedSender<Message>;
pub type UserSockets = Arc<Mutex<HashMap<Uuid, Vec<Sender>>>>;

pub fn run_notifications_pusher<I: IndexStore + 'static>(
    service: AppService,
    index_store: I,
    pusher_count: usize,
    user_sockets: UserSockets,
) {
    info!("Notifications pusher starting");

    let (sender, receiver) = async_channel::bounded::<WsNotification>(50_000);

    let reader = Reader::new(service, index_store, sender);
    tokio::spawn(reader.run());

    for _ in 0..pusher_count {
        let pusher = Pusher::new(receiver.clone(), Arc::clone(&user_sockets));
        tokio::spawn(pusher.run());
    }

    info!("Notifications pusher started");
}

#[derive(Debug)]
pub struct WsNotification {
    recipient: Uuid,
    payload: String,
}

use crate::UserSockets;
use crate::WsNotification;
use async_channel::Receiver;
use axum::extract::ws::Message;

pub struct Pusher {
    receiver: Receiver<WsNotification>,
    user_sockets: UserSockets,
}

impl Pusher {
    pub fn new(receiver: Receiver<WsNotification>, user_sockets: UserSockets) -> Self {
        Self {
            receiver,
            user_sockets,
        }
    }

    pub async fn run(self) {
        while let Ok(notification) = self.receiver.recv().await {
            send_notification(&notification, &self.user_sockets);
        }
    }
}

pub fn send_notification(notification: &WsNotification, state: &UserSockets) {
    let mut user_sockets = state.lock().unwrap();
    if let Some(sockets) = user_sockets.get_mut(&notification.recipient) {
        sockets.retain(|s| !s.is_closed());
        for sender in sockets.iter() {
            let _ = sender.send(Message::Text(notification.payload.clone()));
        }
    }
}

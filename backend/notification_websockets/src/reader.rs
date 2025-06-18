use async_channel::Sender;
use database::AppService;
use index_store::IndexStore;
use tokio::time;
use tracing::{error, info};
use types::error::Error;

use crate::Notification;

pub struct Reader<I: IndexStore> {
    _service: AppService,
    index_store: I,
    sender: Sender<Notification>,
}

impl<I: IndexStore> Reader<I> {
    pub fn new(service: AppService, index_store: I, sender: Sender<Notification>) -> Self {
        Self {
            _service: service,
            index_store,
            sender,
        }
    }

    pub async fn run(self) {
        info!("Notifications reader started");
        let mut interval = time::interval(time::Duration::from_secs(5));
        interval.tick().await;
        loop {
            if self.sender.is_full() {
                error!("Notifications queue is full");
                interval.tick().await;
            } else {
                for _ in 0..30 {
                    if let Err(error) = self.read_notifications().await {
                        error!(?error, "Read notifications failed");
                    }
                    interval.tick().await;
                }
            }
        }
    }

    async fn read_notifications(&self) -> Result<(), Error> {
        let _from_notification_index = self.index_processed_up_to().await? + 1;

        // let notifications = self
        //     .service
        //     .notification
        //     .get_notifications(from_notification_index as i64, 1_000)
        //     .await?;

        // if let Some(latest_notification_index) = notifications.last().map(|e| e.id) {
        //     for notification in &notifications {
        //         let payload = serde_json::to_string(&notification).unwrap();
        //         if self
        //             .sender
        //             .try_send(Notification {
        //                 recipient: notification.user_id.clone(),
        //                 payload,
        //             })
        //             .is_err()
        //         {
        //             return Err("Notifications queue is full".into());
        //         }
        //     }
        //     self.set_index_processed_up_to(latest_notification_index)
        //         .await?;
        // }

        Ok(())
    }

    async fn index_processed_up_to(&self) -> Result<i64, Error> {
        if let Some(index) = self.index_store.get().await? {
            Ok(index)
        } else {
            // let index = self
            //     .service
            //     .notification
            //     .get_latest_notification_index()
            //     .await?;
            // self.set_index_processed_up_to(index).await?;
            Ok(0)
        }
    }

    #[allow(dead_code)]
    async fn set_index_processed_up_to(&self, index: i64) -> Result<(), Error> {
        self.index_store.set(index).await
    }
}

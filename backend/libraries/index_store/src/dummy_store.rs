use crate::IndexStore;
use async_trait::async_trait;
use futures::lock::Mutex;
use std::sync::Arc;
use types::error::Error;

#[derive(Clone, Default)]
pub struct DummyStore {
    indexes_processed_up_to: Arc<Mutex<Option<i64>>>,
}

impl DummyStore {
    pub fn new(notification_index: Option<i64>) -> DummyStore {
        DummyStore {
            indexes_processed_up_to: Arc::new(Mutex::new(notification_index)),
        }
    }
}

#[async_trait]
impl IndexStore for DummyStore {
    async fn get(&self) -> Result<Option<i64>, Error> {
        Ok(*self.indexes_processed_up_to.lock().await)
    }

    async fn set(&self, notification_index: i64) -> Result<(), Error> {
        *self.indexes_processed_up_to.lock().await = Some(notification_index);
        Ok(())
    }
}

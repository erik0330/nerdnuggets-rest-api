mod dummy_store;

pub use dummy_store::DummyStore;

use async_trait::async_trait;
use types::error::Error;

#[async_trait]
pub trait IndexStore: Clone + Send + Sync {
    async fn get(&self) -> Result<Option<i64>, Error>;
    async fn set(&self, index: i64) -> Result<(), Error>;
}

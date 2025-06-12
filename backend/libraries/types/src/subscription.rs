use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Clone, Debug, Validate, Eq, PartialEq)]
pub struct SubscriptionInfo {
    pub endpoint: String,
    pub keys: SubscriptionKeys,
}

#[derive(Serialize, Deserialize, Clone, Debug, Validate, Eq, PartialEq)]
pub struct SubscriptionKeys {
    pub p256dh: String,
    pub auth: String,
}

impl SubscriptionInfo {
    pub fn new(endpoint: String, p256dh: String, auth: String) -> SubscriptionInfo {
        SubscriptionInfo {
            endpoint,
            keys: SubscriptionKeys { p256dh, auth },
        }
    }
}

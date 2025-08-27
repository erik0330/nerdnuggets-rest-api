mod bounty_service;
mod notification_service;
mod prediction_service;
mod prediction_placement_service;
mod project_service;
mod token_service;
mod user_service;
mod util_service;

pub use bounty_service::*;
pub use notification_service::*;
pub use prediction_service::*;
pub use prediction_placement_service::*;
pub use project_service::*;
pub use token_service::*;
pub use user_service::*;
pub use util_service::*;

use crate::DatabasePool;
use std::sync::Arc;
use utils::env::Env;

#[derive(Clone)]
pub struct AppService {
    pub bounty: BountyService,
    pub notification: NotificationService,
    pub prediction: PredictionService,
    pub prediction_placement: PredictionPlacementService,
    pub project: ProjectService,
    pub token: TokenService,
    pub user: UserService,
    pub util: UtilService,
}

impl AppService {
    pub fn init(db: &Arc<DatabasePool>, env: &Env) -> Self {
        Self {
            bounty: BountyService::new(db),
            notification: NotificationService::new(db),
            prediction: PredictionService::new(db),
            prediction_placement: PredictionPlacementService::new(db),
            project: ProjectService::new(db),
            token: TokenService::new(env),
            user: UserService::new(db),
            util: UtilService::new(db),
        }
    }
}

mod admin_service;
mod token_service;
mod user_service;
mod util_service;

pub use admin_service::*;
pub use token_service::*;
pub use user_service::*;
pub use util_service::*;

use crate::DatabasePool;
use std::sync::Arc;
use utils::env::Env;

#[derive(Clone)]
pub struct AppService {
    pub admin: AdminService,
    pub token: TokenService,
    pub user: UserService,
    pub util: UtilService,
}

impl AppService {
    pub fn init(db: &Arc<DatabasePool>, env: &Env) -> Self {
        Self {
            admin: AdminService::init(db),
            token: TokenService::new(env),
            user: UserService::new(db),
            util: UtilService::new(db),
        }
    }
}

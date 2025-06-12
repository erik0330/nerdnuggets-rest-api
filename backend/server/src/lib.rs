mod handler;
mod middleware;
mod routes;
mod state;

use database::DatabasePool;
use routes::routes;
use std::sync::Arc;
use utils::env;

pub async fn run() {
    let env = env::Env::init();
    let connection = DatabasePool::init(&env)
        .await
        .unwrap_or_else(|e| panic!("Database error: {e}"));
    let host = format!("0.0.0.0:{}", env.port);
    axum::Server::bind(&host.parse().unwrap())
        .serve(routes(Arc::new(connection), env).await)
        .await
        .unwrap_or_else(|e| panic!("Server error: {e}"));
}

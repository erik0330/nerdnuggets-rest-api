mod auth;
mod project;
mod public;
mod user;
mod util;

use crate::{
    middleware::{auth as auth_middleware, public as public_middleware},
    state::AppState,
};
use aws_config::{meta::region::RegionProviderChain, Region};
use aws_sdk_s3::config::Credentials;
use axum::{
    extract::DefaultBodyLimit,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        HeaderValue, Method,
    },
    middleware,
    routing::{get, IntoMakeService, Router},
};
use database::DatabasePool;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use utils::env::Env;

pub async fn routes(db_conn: Arc<DatabasePool>, env: Env) -> IntoMakeService<Router> {
    let production = env.production;
    let merged_router = {
        let config = aws_config::load_from_env().await;
        let s3_client: aws_sdk_s3::Client = aws_sdk_s3::Client::new(&config);

        let region = env.email_region.clone();
        let access_key_id = env.aws_access_key_id.clone();
        let secret_access_key = env.aws_secret_access_key.clone();

        let region_provider = RegionProviderChain::first_try(Region::new(region))
            .or_default_provider()
            .or_else(Region::new("us-east-2"));

        let credential = Credentials::new(access_key_id, secret_access_key, None, None, "");
        let shared_config = aws_config::from_env()
            .region(region_provider)
            .credentials_provider(credential)
            .load()
            .await;

        let ses_client = aws_sdk_sesv2::Client::new(&shared_config);

        let app_state = AppState::init(&db_conn, env, s3_client, ses_client);
        Router::new()
            .merge(project::routes())
            .merge(user::routes())
            .merge(util::routes())
            .layer(ServiceBuilder::new().layer(middleware::from_fn_with_state(
                app_state.clone(),
                auth_middleware,
            )))
            .layer(DefaultBodyLimit::max(20 * 1024 * 1024))
            .merge(public::routes().layer(ServiceBuilder::new().layer(
                middleware::from_fn_with_state(app_state.clone(), public_middleware),
            )))
            .merge(auth::routes())
            .with_state(app_state)
            .merge(Router::new().route("/health", get(|| async { "<h1>NERDNUGGETS BACKEND</h1>" })))
            .merge(Router::new().route("/version", get(|| async { "NerdNuggets Backend V1.0.1" })))
    };

    let cors = CorsLayer::new()
        .allow_origin(if production {
            vec![
                "https://www.nerdnuggets.com"
                    .parse::<HeaderValue>()
                    .unwrap(),
                "https://www.nerdbunny.com".parse::<HeaderValue>().unwrap(),
            ]
        } else {
            vec![
                "https://nerdnuggets.vercel.app"
                    .parse::<HeaderValue>()
                    .unwrap(),
                "http://localhost:3000".parse::<HeaderValue>().unwrap(),
                "http://localhost:3001".parse::<HeaderValue>().unwrap(),
                "http://localhost:3002".parse::<HeaderValue>().unwrap(),
            ]
        })
        .allow_methods([
            Method::POST,
            Method::GET,
            Method::PATCH,
            Method::DELETE,
            Method::PUT,
            Method::HEAD,
            Method::OPTIONS,
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app_router = Router::new().nest("/api/v2", merged_router).layer(cors);

    app_router.into_make_service()
}

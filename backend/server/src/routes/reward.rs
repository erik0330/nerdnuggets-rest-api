use crate::{
    handler::reward_handler::{
        claim_reward, create_reward, delete_reward, get_all_rewards, get_reward_by_id,
        get_rewards_by_type, get_rewards_by_wallet_address, get_user_claimed_rewards,
        get_user_reward_stats, get_user_rewards, get_user_rewards_count,
        get_user_unclaimed_rewards, get_user_unclaimed_rewards_count, update_reward_points,
    },
    state::AppState,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        // Basic CRUD operations
        .route("/rewards/:id", get(get_reward_by_id))
        .route("/rewards/:id", put(update_reward_points))
        .route("/rewards/:id", delete(delete_reward))
        // User-specific reward operations
        .route("/rewards/user/me", get(get_user_rewards))
        .route(
            "/rewards/user/me/unclaimed",
            get(get_user_unclaimed_rewards),
        )
        .route("/rewards/user/me/claimed", get(get_user_claimed_rewards))
        .route("/rewards/user/me/stats", get(get_user_reward_stats))
        .route("/rewards/user/me/count", get(get_user_rewards_count))
        .route(
            "/rewards/user/me/unclaimed/count",
            get(get_user_unclaimed_rewards_count),
        )
        .route("/rewards/:reward_id/claim", post(claim_reward))
        // Filtering and search
        .route("/rewards/type/:reward_type", get(get_rewards_by_type))
        .route(
            "/rewards/wallet/:wallet_address",
            get(get_rewards_by_wallet_address),
        )
        // Admin operations
        .route("/rewards", post(create_reward))
        .route("/rewards/all", get(get_all_rewards))
}

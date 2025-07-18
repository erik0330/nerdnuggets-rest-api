use crate::{
    handler::bounty_handler::{
        create_bidder_chat, create_bounty, delete_bounty, get_bids, get_bounty_chat_list,
        get_bounty_chat_numbers, get_bounty_chats, get_bounty_comments, get_my_bids,
        get_my_bounty_stats, mark_chat_as_read, reject_bid, review_bounty, select_as_winner,
        send_bounty_chat, submit_bid, submit_bounty_comment, update_bounty,
    },
    state::AppState,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/bounty", post(create_bounty))
        .route("/bounty/:id", put(update_bounty))
        .route("/bounty/:id", delete(delete_bounty))
        .route("/bounty/:id/bid", get(get_bids))
        .route("/bounty/:id/bid", post(submit_bid))
        .route("/bounty/:id/comment", get(get_bounty_comments))
        .route("/bounty/:id/comment", post(submit_bounty_comment))
        .route("/bounty/:id/review", post(review_bounty))
        .route("/bounty/stats", get(get_my_bounty_stats))
        .route("/bounty/chat/list", get(get_bounty_chat_list))
        .route("/bounty/chat", get(get_bounty_chats))
        .route("/bounty/:id/chat", post(send_bounty_chat))
        .route("/bounty/:id/chat/numbers", get(get_bounty_chat_numbers))
        .route("/bounty/:id/chat/:chat_number/read", put(mark_chat_as_read))
        .route(
            "/bounty/:id/chat/bidder/:bidder_id",
            post(create_bidder_chat),
        )
        .route("/bid/:id/reject", put(reject_bid))
        .route("/bid/:id/win", put(select_as_winner))
        .route("/bid/me", get(get_my_bids))
}

use crate::{
    handler::bounty_handler::{
        cancel_bounty, create_bidder_chat, create_bounty, delete_bounty,
        finalize_bounty_work_submission, get_bid_milestone_submissions, get_bounty_chat_numbers,
        get_bounty_chats, get_bounty_comments, get_bounty_work_submission, get_my_bids,
        get_my_bounty_stats, get_winning_bid_milestones, handle_bounty_action, mark_chat_as_read,
        reject_bid, review_bid_milestone_submission, review_bounty, review_bounty_work_submission,
        save_bounty_work, select_as_winner, send_bounty_chat, submit_bid,
        submit_bid_milestone_work, submit_bounty_comment, update_bounty,
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
        .route("/bounty/:id/bid", post(submit_bid))
        .route(
            "/bounty/:id/bid-milestones",
            get(get_winning_bid_milestones),
        )
        .route("/bounty/:id/comment", get(get_bounty_comments))
        .route("/bounty/:id/comment", post(submit_bounty_comment))
        .route("/bounty/:id/review", post(review_bounty))
        .route("/bounty/:id/action", post(handle_bounty_action))
        .route("/bounty/:id/cancel", post(cancel_bounty))
        .route("/bounty/stats", get(get_my_bounty_stats))
        .route("/bounty/chat", get(get_bounty_chats))
        // .route("/bounty/chat/list", get(get_bounty_chat_list))
        .route("/bounty/chat/numbers", get(get_bounty_chat_numbers))
        .route("/bounty/chat", post(send_bounty_chat))
        .route("/bounty/chat/:chat_number/read", put(mark_chat_as_read))
        .route(
            "/bounty/:id/chat/bidder/:bidder_id",
            post(create_bidder_chat),
        )
        .route("/bid/:id/reject", put(reject_bid))
        .route("/bid/:id/win", put(select_as_winner))
        .route("/bid/me", get(get_my_bids))
        .route("/bid/:id/save-work", post(save_bounty_work))
        .route(
            "/bounty-work/:submission_id",
            get(get_bounty_work_submission),
        )
        .route(
            "/bounty-work/:submission_id/finalize",
            post(finalize_bounty_work_submission),
        )
        .route(
            "/bounty-work/:submission_id/review",
            post(review_bounty_work_submission),
        )
        .route(
            "/bounty/bid-milestone/:bid_milestone_id/submit-work",
            post(submit_bid_milestone_work),
        )
        .route(
            "/bounty/bid-milestone/:bid_milestone_id/submissions",
            get(get_bid_milestone_submissions),
        )
        .route(
            "/bounty/bid-milestone-submission/:submission_id/review",
            post(review_bid_milestone_submission),
        )
}

mod api_error;
mod db_error;
mod request_error;
mod token_error;
mod upload_err;
mod user_error;

pub use api_error::*;
pub use db_error::*;
pub use request_error::*;
pub use token_error::*;
pub use upload_err::*;
pub use user_error::*;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

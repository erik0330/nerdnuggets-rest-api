use crate::response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UploadError {
    #[error("There was an issue processing the data. Please try again.")]
    FailedProcessData,
    #[error("No file provided. Please upload a file.")]
    NoFileProvided,
    #[error("Something went wrong: {0}.")]
    SomethingWentWrong(String),
}

impl IntoResponse for UploadError {
    fn into_response(self) -> Response {
        let status_code = match self {
            UploadError::FailedProcessData => StatusCode::INTERNAL_SERVER_ERROR,
            UploadError::NoFileProvided => StatusCode::BAD_REQUEST,
            UploadError::SomethingWentWrong(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}

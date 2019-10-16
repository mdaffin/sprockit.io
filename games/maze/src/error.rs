use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display, PartialEq)]
pub enum ServiceError {
    #[display(fmt = "missing session token")]
    MissingSessionToken,
    #[display(fmt = "token is not a valid utf-8 string")]
    InvalidTokenUTF8,
    #[display(fmt = "token is not a valid UUID")]
    InvalidTokenUUID,
    #[display(fmt = "session not found")]
    SessionNotFound,
}

#[derive(Debug, Serialize)]
struct ErrorResponse<'a> {
    pub error: &'a str,
    pub help: &'a str,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::MissingSessionToken => {
                HttpResponse::BadRequest().json(ErrorResponse{
                    error: &format!("{}", self),
                    help: "The token can be set with the header `X-TOKEN`Session tokens are obtained by sending a post request to /start",
                })
            }
            ServiceError::InvalidTokenUTF8 => {
                HttpResponse::BadRequest().json(ErrorResponse{
                    error: &format!("{}", self),
                    help: "The token must be a valid utf-8 string. Session tokens are obtained by sending a post request to /start",
                })
            }
            ServiceError::InvalidTokenUUID => {
                HttpResponse::BadRequest().json(ErrorResponse{
                    error: &format!("{}", self),
                    help: "The token must be a valid utf-8 string. Session tokens are obtained by sending a post request to /start",
                })
            }
            ServiceError::SessionNotFound => {
                HttpResponse::BadRequest().json(ErrorResponse{
                    error: &format!("{}", self),
                    help: "No session was found for the given token. Session tokens are obtained by sending a post request to /start",
                })
            }
        }
    }
}

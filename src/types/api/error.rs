use actix_web::{error::BlockingError, http::StatusCode, HttpResponse, ResponseError};
use ethers::{prelude::ContractError, providers::Middleware};
use log::debug;

#[derive(Debug)]
pub enum Error {
    Internal,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().body("Error")
    }
}

impl From<BlockingError> for Error {
    #[inline]
    fn from(_: BlockingError) -> Self {
        Self::Internal
    }
}

impl From<std::io::Error> for Error {
    #[inline]
    fn from(e: std::io::Error) -> Self {
        debug!("std::io::Error: {e:?}");
        Self::Internal
    }
}

impl<M: Middleware> From<ContractError<M>> for Error {
    #[inline]
    fn from(e: ContractError<M>) -> Self {
        debug!("Contract error: {e:?}");
        Self::Internal
    }
}

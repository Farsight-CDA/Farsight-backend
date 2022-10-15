use actix_web::{error::BlockingError, http::StatusCode, HttpResponse, ResponseError};

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
    fn from(_: std::io::Error) -> Self {
        Self::Internal
    }
}

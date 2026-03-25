
use actix_web::http::StatusCode;

#[derive(Debug)]
pub enum AppError {
    Unauthorized,   // 401
    Forbidden,      // 403
    NotFound,       // 404
    Internal,       // 500
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::Unauthorized => write!(f, "Unauthorized"),
            AppError::Forbidden    => write!(f, "Forbidden"),
            AppError::NotFound     => write!(f, "Not Found"),
            AppError::Internal     => write!(f, "Internal Server Error"),
        }
    }
}

impl actix_web::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::Forbidden    => StatusCode::FORBIDDEN,
            AppError::NotFound     => StatusCode::NOT_FOUND,
            AppError::Internal     => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

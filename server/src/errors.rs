use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("The room is already full")]
    TooManyUsers,

    #[error("{0} already exists")]
    AlreadyExists(String),
    #[error("{0} does not exist")]
    DoesNotExist(String),
    #[error("not authenticated")]
    NotAuthenticated,
    #[error("unknown error")]
    Unknown,
    #[error("{0} header is required")]
    HeaderMissed(String),
    #[error("{0} header should be a valid {1}")]
    HeaderIsInvalid(String, String),
}

impl From<AppError> for (StatusCode, String) {
    fn from(e: AppError) -> Self {
        match &e {
            AppError::AlreadyExists(_) => (StatusCode::CONFLICT, e.to_string()),
            AppError::NotAuthenticated => (StatusCode::UNAUTHORIZED, e.to_string()),
            AppError::DoesNotExist(_) => (StatusCode::NOT_FOUND, e.to_string()),
            AppError::HeaderMissed(_) | AppError::HeaderIsInvalid(..) => {
                (StatusCode::BAD_REQUEST, e.to_string())
            }
            _ => {
                tracing::error!("{e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Something went wrong".to_owned(),
                )
            }
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

use axum::{
    body::Body,
    http::{HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use nbb_renderer::TeraError;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::time::SystemTimeError;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum WebServerError {
    NotFound,
    TeraError(TeraError),
    JoinError(JoinError),
    IoError(std::io::Error),
    SystemTimeError(SystemTimeError),
}

impl Display for WebServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WebServerError::NotFound => f.write_str("page not found"),
            WebServerError::TeraError(e) => {
                f.write_str(format!("tera rendering engine error: this is a bug\n{}", e).as_str())
            }
            WebServerError::JoinError(e) => {
                f.write_str(format!("background tokio task panicked: {}", e).as_str())
            }
            WebServerError::IoError(e) => {
                f.write_str(format!("filesystem IO error: {}", e).as_str())
            }
            WebServerError::SystemTimeError(e) => {
                f.write_str(format!("system time behind unix epoch: {}", e).as_str())
            }
        }
    }
}

impl Error for WebServerError {}

impl From<TeraError> for WebServerError {
    fn from(e: TeraError) -> Self {
        Self::TeraError(e)
    }
}

impl From<JoinError> for WebServerError {
    fn from(e: JoinError) -> Self {
        Self::JoinError(e)
    }
}

impl From<std::io::Error> for WebServerError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl From<SystemTimeError> for WebServerError {
    fn from(e: SystemTimeError) -> Self {
        Self::SystemTimeError(e)
    }
}

impl IntoResponse for WebServerError {
    fn into_response(self) -> Response {
        let (status, response_body, content_type) = match self {
            WebServerError::NotFound => (
                StatusCode::NOT_FOUND,
                Body::new(nbb_renderer::render_404()),
                HeaderValue::from_static("text/html"),
            ),
            WebServerError::TeraError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::new(format!(
                    "fatal internal server error while rendering this page: {}\n\
                    this is a bug: please report it at https://github.com/tazz4843/nbb/issues/new",
                    e
                )),
                HeaderValue::from_static("text/plain"),
            ),
            WebServerError::JoinError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::new(format!(
                    "background tokio task panicked: {}\n\
                    this is a bug: please report it at https://github.com/tazz4843/nbb/issues/new",
                    e
                )),
                HeaderValue::from_static("text/plain"),
            ),
            WebServerError::IoError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::new(format!("filesystem IO error: {}", e)),
                HeaderValue::from_static("text/plain"),
            ),
            WebServerError::SystemTimeError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Body::new(format!("system time behind unix epoch: {}", e)),
                HeaderValue::from_static("text/plain"),
            ),
        };

        Response::builder()
            .header("Content-Type", content_type)
            .status(status)
            .body(response_body)
            .expect("unreachable code: invalid Content-Type or status code")
    }
}

//! Crate-wide error type.

use thiserror::Error;

pub type OctostudioResult<T> = Result<T, OctostudioError>;

#[derive(Debug, Error)]
pub enum OctostudioError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid id: {0}")]
    InvalidId(String),

    #[error("plan kind not set")]
    NoPlanKind,

    #[error("export format not supported for plan kind {0:?}")]
    ExportUnsupported(PlanKind),

    #[error("ai error: {0}")]
    Ai(String),

    #[error("http error: {0}")]
    Http(String),

    #[error("config error: {0}")]
    Config(String),

    #[error("migration: {0}")]
    Migration(String),

    #[error("not implemented: {0}")]
    NotImplemented(&'static str),
}

use crate::enums::PlanKind;

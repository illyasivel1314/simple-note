use serde::{Serialize, Serializer};
use std::error::Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    FileError(#[from] FileError),

    #[error("{0}")]
    TokioError(#[from] TokioError),

    #[error("{0}")]
    JsonError(#[from] JsonError),

    #[error("{0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("{0}")]
    TauriError(#[from] TauriError),
}

#[derive(Serialize)]
#[serde(tag = "kind", content = "message")]
#[serde(rename_all = "camelCase")]
pub enum AppErrorKind {
    FileError(String),
    TokioError(String),
    JsonError(String),
    DatabaseError(String),
    TauriError(String)
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let error_message = self.to_string();
        let error_kind = match self {
            AppError::FileError(_) => AppErrorKind::FileError(error_message),
            AppError::TokioError(_) => AppErrorKind::TokioError(error_message),
            AppError::JsonError(_) => AppErrorKind::JsonError(error_message),
            AppError::DatabaseError(_) => AppErrorKind::DatabaseError(error_message),
            AppError::TauriError(_) => AppErrorKind::TauriError(error_message)
        };
        error_kind.serialize(serializer)
    }
}

// 文件相关错误
#[derive(Error, Debug)]
pub enum FileError {
    #[error("[[Error]] file dir not found: {0}")]
    FileDirNotFound(std::io::Error),
    #[error("[[Error]] file not found: {0}")]
    FileNotFound(std::io::Error),
    #[error("[[Error]] file read error: {0}")]
    FileReadError(std::io::Error),
    #[error("[[Error]] file list traverse error: {0}")]
    FileTraverseError(std::io::Error),
    #[error("[[Error]] parent file create error: {0}")]
    FileDirCreateError(std::io::Error),
    #[error("[[Error]] file create error: {0}")]
    FileCreateError(std::io::Error),
}

#[derive(Error, Debug)]
pub enum TokioError {
    #[error("[[Error]] tokio runtime error: {0}")]
    TokioRunTimeError(#[from] tokio::task::JoinError),
}

#[derive(Error, Debug)]
pub enum JsonError {
    #[error("[[Error]] json transform error: {0}")]
    JsonTransformError(#[from] serde_json::Error),
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("[[Error]] database connect error: {0}")]
    DatabaseConnectionError(r2d2::Error),

    #[error("[[Error]] database file migrations error: {0}")]
    DatabaseMigrationsError(Box<dyn Error + Send + Sync>),

    #[error("[[Error]] database pool create error: {0}")]
    DatabasePoolCreateError(r2d2::Error),

    #[error("[[Error]] database operation error: {0}")]
    DatabaseOperationError(#[from] diesel::result::Error),
}

#[derive(Error, Debug)]
pub enum TauriError {
    #[error("[[Error]] tauri error: {0}")]
    TauriSystemError(tauri::Error),
    
    #[error("[[Error]] tauri error: {0}")]
    TauriFileError(tauri::Error),
}
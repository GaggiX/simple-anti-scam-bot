use teloxide::prelude::*;
use teloxide::requests::Request;
use teloxide::ApiErrorKind;
use teloxide::KnownApiErrorKind;

use std::fmt::Display;

pub async fn handle_error<T: Request, K: Request>(request: T, error: KnownApiErrorKind, error_handler: K) {
    if let Err(RequestError::ApiError {
        kind: ApiErrorKind::Known(known_error),
        ..
    }) = request.send().await
    {
        if known_error == error {
            if let Err(e) = error_handler.send().await {
                eprintln!("{}", e)
            }
        }
    }
}

#[derive(Debug)]
pub enum ConfigError {
    NoGroup,
    NoKeywordsGroup,
    ActionError(String),
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoGroup => write!(f, "No group found in the config file, you must insert at least one group"),
            Self::NoKeywordsGroup => write!(f, "No keywords-group found in the config file, you must insert at least one keywords-group"),
            Self::ActionError(s) => write!(f, "Only \"kick\" or \"ban\" are valid actions, got: {}", s)
        }
    }
}

impl std::error::Error for ConfigError {}

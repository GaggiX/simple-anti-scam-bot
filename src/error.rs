use teloxide::prelude::*;
use teloxide::ApiErrorKind;
use teloxide::KnownApiErrorKind;

use std::fmt::Display;

pub fn is_error<T>(result: ResponseResult<T>, error: KnownApiErrorKind) -> bool {
    if let Err(err) = result {
        if let RequestError::ApiError { kind, .. } = err {
            if let ApiErrorKind::Known(known_error) = kind {
                return known_error == error;
            }
        }
    }
    false
}

#[derive(Debug)]
pub enum ConfigError {
    NoGroup,
    NoKeywordsGroup,
}

impl Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoGroup => write!(f, "No group found in the config file, you must insert at least one group"),
            Self::NoKeywordsGroup => write!(f, "No keywords-group found in the config file, you must insert at least one keywords-group")
        }
    }
}

impl std::error::Error for ConfigError {}

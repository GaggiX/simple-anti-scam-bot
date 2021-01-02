use crate::error::ConfigError;
use serde_derive::Deserialize;
use teloxide::types::{Chat, ChatKind, PublicChatKind};

use std::collections::HashMap;
use std::fs::read_to_string;

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_string = read_to_string("config.toml")?;
    let config_toml: ConfigToml = toml::from_str(&config_string)?;
    Ok(Config::new(config_toml)?)
}

pub struct Config {
    pub token: Option<String>,
    groups: HashMap<String, Option<i64>>,
    pub keywords_groups: Vec<KeywordGroups>,
}

impl Config {
    fn new(config_toml: ConfigToml) -> Result<Self, ConfigError> {
        if config_toml.groups.len() == 0 {
            return Err(ConfigError::NoGroup);
        }

        if config_toml.keywords_groups.len() == 0 {
            return Err(ConfigError::NoKeywordsGroup);
        }

        let mut groups = HashMap::new();
        for Group { username, log_id } in config_toml.groups {
            groups.insert(username, log_id);
        }

        Ok(Self {
            token: config_toml.token,
            groups,
            keywords_groups: config_toml.keywords_groups,
        })
    }

    pub fn is_chat_legit(&self, chat: &Chat) -> bool {
        self.get_from_config(chat) != None
    }

    pub fn get_log_id(&self, chat: &Chat) -> Option<i64> {
        *self.get_from_config(chat).unwrap()
    }

    fn get_from_config(&self, chat: &Chat) -> Option<&Option<i64>> {
        if let ChatKind::Public(ref chat_public) = chat.kind {
            if let PublicChatKind::Supergroup(ref supergroup) = chat_public.kind {
                if let Some(ref username) = supergroup.username {
                    return self.groups.get(username)
                }
            }
        }
        None
    }
}

#[derive(Debug, Deserialize)]
struct ConfigToml {
    pub token: Option<String>,
    groups: Vec<Group>,
    #[serde(rename = "keywords-groups")]
    keywords_groups: Vec<KeywordGroups>,
}

#[derive(Debug, Deserialize)]
struct Group {
    username: String,
    log_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct KeywordGroups {
    pub keywords: Vec<String>,
}

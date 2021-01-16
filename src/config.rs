use crate::error::ConfigError;
use serde_derive::Deserialize;
use teloxide::types::{Chat, ChatKind, PublicChatKind};

use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::read_to_string;

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_string = read_to_string("config.toml")?;
    let config_toml: ConfigToml = toml::from_str(&config_string)?;
    Ok(Config::new(config_toml)?)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Kick,
    Ban,
    Ignore
}

impl TryFrom<String> for Action {
    type Error = ConfigError;

    fn try_from(s: String) -> Result<Self, ConfigError> {
        match s.to_lowercase().as_ref() {
            "kick" => Ok(Action::Kick),
            "ban" => Ok(Action::Ban),
            "none" => Ok(Action::Ignore),
            _ => Err(ConfigError::ActionError(s)),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub token: Option<String>,
    pub default_action: Option<Action>,
    groups: HashMap<String, (Option<i64>, Option<Action>)>,
    pub keywords_groups: Vec<KeywordGroups>,
}

impl Config {
    fn new(mut config_toml: ConfigToml) -> Result<Self, ConfigError> {
        if config_toml.groups.len() == 0 {
            return Err(ConfigError::NoGroup);
        }

        if config_toml.keywords_groups.len() == 0 {
            return Err(ConfigError::NoKeywordsGroup);
        }

        let mut groups = HashMap::new();
        for Group {
            username,
            log_id,
            action,
        } in config_toml.groups
        {
            groups.insert(
                username,
                (log_id, action.map(|s| Action::try_from(s)).transpose()?),
            );
        }

        config_toml
            .keywords_groups
            .iter_mut()
            .for_each(|keywords_group| {
                keywords_group
                    .keywords
                    .iter_mut()
                    .for_each(|keyword| *keyword = keyword.to_lowercase())
            });

        Ok(Self {
            token: config_toml.token,
            default_action: config_toml
                .default_action
                .map(|s| Action::try_from(s))
                .transpose()?,
            groups,
            keywords_groups: config_toml.keywords_groups,
        })
    }

    pub fn is_chat_legit(&self, chat: &Chat) -> bool {
        self.get_from_config(chat) != None
    }

    pub fn get_log_id(&self, chat: &Chat) -> Option<i64> {
        self.get_from_config(chat).unwrap().0
    }

    pub fn get_group_action(&self, chat: &Chat) -> Option<Action> {
        self.get_from_config(chat).unwrap().1
    }

    fn get_from_config(&self, chat: &Chat) -> Option<&(Option<i64>, Option<Action>)> {
        if let ChatKind::Public(ref chat_public) = chat.kind {
            if let PublicChatKind::Supergroup(ref supergroup) = chat_public.kind {
                if let Some(ref username) = supergroup.username {
                    return self.groups.get(username);
                }
            }
        }
        None
    }
}

#[derive(Debug, Deserialize)]
struct ConfigToml {
    token: Option<String>,
    default_action: Option<String>,
    groups: Vec<Group>,
    #[serde(rename = "keywords-groups")]
    keywords_groups: Vec<KeywordGroups>,
}

#[derive(Debug, Deserialize)]
struct Group {
    username: String,
    log_id: Option<i64>,
    action: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct KeywordGroups {
    pub keywords: Vec<String>,
}

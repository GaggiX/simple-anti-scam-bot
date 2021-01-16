mod config;
mod error;

use config::Config;
use error::handle_error;
use lazy_static::lazy_static;
use teloxide::prelude::*;
use teloxide::BotBuilder;
use teloxide::KnownApiErrorKind;
use tokio::fs::File;

use std::env;
use std::fs::remove_file;
use std::process::exit;
use std::process::Command;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

lazy_static! {
    static ref CONFIG: Arc<Config> = Arc::new({
        match config::read_config() {
            Ok(conf) => conf,
            Err(e) => {
                eprintln!("Error reading config file: {}", e);
                exit(1)
            }
        }
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    teloxide::enable_logging!();

    let bot = if let Some(token) = &CONFIG.token {
        BotBuilder::new().token(token).build()
    } else {
        Bot::from_env()
    };

    teloxide::repl(bot, |message| async move {
        let chat = &message.update.chat;
        let bot = &message.bot;
        let config = Arc::clone(&CONFIG);

        if chat.is_channel() || chat.is_private() || !config.is_chat_legit(chat) {
            return ResponseResult::<()>::Ok(());
        }

        let photo_id = match &message.update.photo() {
            Some(photos) => &photos.last().unwrap().file_id,
            None => return ResponseResult::<()>::Ok(()),
        };

        let image_name = save_image(&message, photo_id).await.unwrap();
        let scam = scam_test(image_name, &config).unwrap();

        if scam {
            handle_error(
                message.delete_message(),
                KnownApiErrorKind::MessageCantBeDeleted,
                message.reply_to("I can't delete this photo\nI don't have enough rights."),
            )
            .await;

            if let Some(log_id) = config.get_log_id(chat) {
                bot.send_message(log_id, "Removed scam from group")
                    .send()
                    .await?;
            }

            match config.get_group_action(chat).or(config.default_action) {
                Some(config::Action::Kick) => {
                    handle_error(
                        bot.unban_chat_member(chat.id, message.update.from().unwrap().id),
                        KnownApiErrorKind::NotEnoughRightsToRestrict,
                        bot.send_message(
                            chat.id,
                            "I can't kick the user\nI don't have enough rights.",
                        ),
                    )
                    .await
                }
                Some(config::Action::Ban) => {
                    handle_error(
                        bot.kick_chat_member(chat.id, message.update.from().unwrap().id),
                        KnownApiErrorKind::NotEnoughRightsToRestrict,
                        bot.send_message(
                            chat.id,
                            "I can't ban the user\nI don't have enough rights.",
                        ),
                    )
                    .await
                }
                Some(config::Action::Ignore) | None => (),
            }
        }

        ResponseResult::<()>::Ok(())
    })
    .await;

    Ok(())
}

async fn save_image(
    message: &UpdateWithCx<Message>,
    photo_id: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let photo_path = message.bot.get_file(photo_id).send().await?.file_path;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let image_name = format!("image-{}", timestamp);
    let mut file = File::create(&image_name).await?;

    message.bot.download_file(&photo_path, &mut file).await?;

    Ok(image_name)
}

fn scam_test(image_name: String, config: &Config) -> Result<bool, Box<dyn std::error::Error>> {
    let text = String::from_utf8(
        Command::new("tesseract")
            .args(&[&image_name, "-"])
            .output()?
            .stdout,
    )?
    .to_lowercase();

    remove_file(image_name)?;
    Ok(config.keywords_groups.iter().any(|keywords_group| {
        keywords_group
            .keywords
            .iter()
            .all(|keyword| text.contains(keyword))
    }))
}

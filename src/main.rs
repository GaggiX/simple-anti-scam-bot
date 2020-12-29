use reqwest;
use teloxide::prelude::*;

use std::env;
use std::fs::{File, remove_file};
use std::io::Write;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();

    let bot = Bot::from_env();

    teloxide::repl(bot, |message| async move {
        let chat = &message.update.chat;
        if chat.is_channel() || chat.is_private() {
            return ResponseResult::<()>::Ok(());
        }
        let photo_id = match &message.update.photo() {
            Some(photos) => &photos.last().unwrap().file_id,
            None => return ResponseResult::<()>::Ok(()),
        };

        let image_name = save_image(&message, photo_id).await.unwrap();
        let scam = scam_test(image_name).unwrap();

        if scam {
            message.delete_message().send().await?;
        }
        
        ResponseResult::<()>::Ok(())
    })
    .await;
}

async fn save_image(
    message: &UpdateWithCx<Message>,
    photo_id: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let photo_path = message.bot.get_file(photo_id).send().await?.file_path;
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();

    let bytes_image = reqwest::get(&format!(
        "https://api.telegram.org/file/bot{}/{}",
        env::var("TELOXIDE_TOKEN").expect("TELOXIDE_TOKEN env variable missing"),
        photo_path
    ))
    .await?
    .bytes()
    .await?;

    let image_name = format!("image-{}", timestamp);
    let mut file = File::create(&image_name)?;
    file.write_all(&bytes_image)?;
    Ok(image_name)
}

fn scam_test(image_name: String) -> Result<bool, Box<dyn std::error::Error>> {
    let text = String::from_utf8(
        Command::new("tesseract")
            .args(&[&image_name, "-"])
            .output()?
            .stdout,
    )?;

    remove_file(image_name)?;
    Ok(text.contains("bitcoin") && text.contains("blockchain") && text.contains("giveaway"))
}

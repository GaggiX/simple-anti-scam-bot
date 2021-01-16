# Anti-scam bot

This bot can automatically remove scam images from your supergroup Telegram chat.

## Build

- install `tesseract`, make sure it is in your `PATH`
- if you haven't already, you must install [the Rust toolchain](https://www.rust-lang.org/tools/install)
- compile and run:
```sh
cargo run --release
```
Or just build:
```sh
cargo build --release
```

## Configuration

`config.toml` must be present in the current working directory for the program to run.

```toml
# Optional, you can omit it if you already set TELOXIDE_TOKEN environmental variable
token = "<INSERT YOUR TOKEN HERE>"
# Optional, `action` will be executed after a scam image is detected. This field is applied to all groups where it is not specified, valid keywords are "kick" or "ban" 
# default_action = "kick"

# Required, at least one group must be specified
[[groups]]
# Required
username = "<INSERT GROUP'S USERNAME>"
# Optional, insert chat id where the bot will log the removed messages 
# log_id = 123456789
# Optional, valid keywords "kick" or "ban", special keyword "none" to only shadow the `default_action` 
# action = "none"

# [[groups]]
# username = "group_example"
# log_id = 123456789
# action = "ban"

# Required, at least one keywords group must be specified
[[keywords-groups]]
# Required, an image must contain all the keywords in this field in order to be removed
keywords = ["bitcoin", "blockchain", "giveaway"]

# [[keywords-groups]]
# keywords = ["pear", "apple", "banana"]
```
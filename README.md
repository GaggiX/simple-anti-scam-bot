# Anti-scam bot

This bot can automatically remove scam images from your supergroup Telegram chat.

## Configuration

`config.toml` must be present in the current working directory for the program to run.

```toml
# Optional, you can omit it if you already set TELOXIDE_TOKEN environmental variable
token = "<INSERT YOUR TOKEN HERE>"

# Required, at least one group must be specified
[[groups]]
# Required
username = "<INSERT GROUP'S USERNAME>"
# Optional, insert chat id where the bot will log the removed messages 
# log_id = 123456789

# [[groups]]
# username = "group_example"
# log_id = 123456789

# Required, at least one keywords group must be specified
[[keywords-groups]]
# Required, an image must contain all the keywords in this field in order to be removed
keywords = ["bitcoin", "blockchain", "giveaway"]

# [[keywords-groups]]
# keywords = ["pear", "apple", "banana"]
```
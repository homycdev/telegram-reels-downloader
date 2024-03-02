use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
rename_rule = "lowercase",
description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "download this reel.")]
    Download(String),
}

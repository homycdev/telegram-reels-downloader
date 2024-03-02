use teloxide::prelude::Message;
use teloxide::types::InputFile;
use teloxide::Bot;
use teloxide::{prelude::*, utils::command::BotCommands};

use crate::domain::command::Command;
use crate::domain::types::types::MyResponseResult;
use crate::service::url_fetcher;

pub async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> MyResponseResult {
    match cmd {
        Command::Help => help(bot, msg).await,
        Command::Download(url) => download(bot, msg, url).await,
    }
}

async fn download(bot: Bot, msg: Message, url: String) -> MyResponseResult {
    let url = url_fetcher::fetch_url(url).await?;

    let result = bot.send_video(msg.chat.id, InputFile::file(url)).await;
    match &result {
        Ok(_) => (),
        Err(e) => {
            log::error!("Received an error while sending. {}", e.to_string())
        }
    };

    Ok(())
}

async fn help(bot: Bot, msg: Message) -> MyResponseResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

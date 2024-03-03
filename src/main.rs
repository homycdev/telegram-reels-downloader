use crate::command::model::Command;
use teloxide::prelude::*;

mod command;
mod ig;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let bot = Bot::from_env();

    Command::repl(bot, command::handler::command_handler).await;
}

use teloxide::prelude::*;

use crate::domain::command::Command;

mod domain;
mod handler;
mod service;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let bot = Bot::from_env();

    Command::repl(bot, handler::command::command_handler).await;
}

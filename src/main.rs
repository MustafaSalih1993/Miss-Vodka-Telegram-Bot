mod commands;
mod webhook;

use crate::commands::answer;
use teloxide::prelude::*;
use webhook::webhook;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting Miss-VodkaBot...");

    let bot = Bot::from_env();

    let cloned_bot = bot.clone();
    teloxide::commands_repl_with_listener(bot, "Miss_VodkaBot", answer, webhook(cloned_bot).await)
        .await
}

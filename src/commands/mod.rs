mod gif;

use gif::get_gif;
use teloxide::prelude::*;
use teloxide::{requests::RequestWithFile, types::InputFile, utils::command::BotCommand};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Recieve a 'Pong' from the bot!")]
    Ping,
    #[command(description = "Tell the bot to say anything you type after /echo")]
    Echo(String),
    #[command(description = "Search a gif photo")]
    Gif(String),
    #[command(description = "display this text.")]
    Help,
}

#[allow(unreachable_patterns)]
pub async fn answer(cx: UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Echo(s) => cx.answer_str(s).await?,
        Command::Ping => cx.answer_str("Pong!").await?,
        Command::Gif(s) => {
            if s.is_empty() {
                cx.answer_str("Consider Giving The Photo A Name You Fucking Asshole!")
                    .await?
            } else {
                let gif_url = get_gif(s).await;
                if gif_url.is_none() {
                    cx.answer_str("Some shit happend, try again!").await?
                } else {
                    cx.answer_animation(InputFile::Url(gif_url.unwrap()))
                        .send()
                        .await
                        .unwrap()?
                }
            }
        }
    };

    Ok(())
}

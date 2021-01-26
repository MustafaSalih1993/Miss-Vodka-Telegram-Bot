mod gif;
mod lyrics;
mod quote;

use gif::get_gif;
use lyrics::get_lyrics;
use quote::get_random_quote;
use teloxide::prelude::*;
use teloxide::{requests::RequestWithFile, types::InputFile, utils::command::BotCommand};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Recieve a 'Pong' from the bot!")]
    Ping,
    #[command(description = "Tell the bot to say anything you type after /echo")]
    Echo(String),
    #[command(description = "Misc:")]
    #[command(description = "get lyrics of a song, in this format \"artist - song\"")]
    #[command(description = "Random Quote")]
    Quote,
    Lyrics(String),
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
        Command::Lyrics(s) => {
            let lyrics_data = get_lyrics(s).await;
            if lyrics_data.is_some() {
                cx.answer(lyrics_data.unwrap())
                    .parse_mode(teloxide::types::ParseMode::HTML)
                    .send()
                    .await?
            } else {
                cx.answer_str("something wrong, try somthing else").await?
            }
        }
        Command::Quote => {
            let quote_data = get_random_quote().await;
            if quote_data.is_none() {
                cx.answer_str("something wrong, try somthing else").await?
            } else {
                cx.answer(quote_data.unwrap())
                    .parse_mode(teloxide::types::ParseMode::HTML)
                    .send()
                    .await?
            }
        }
    };

    Ok(())
}

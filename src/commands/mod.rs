mod common;
mod gif;
mod lyrics;
mod quote;
mod translate;
mod tumbler;
mod video;
mod wiki;

use gif::handle_gif;
use lyrics::handle_lyrics;
use quote::handle_quote;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommand;
use translate::handle_en;
use translate::handle_pr;
use tumbler::handle_tumbler;
use video::handle_video;
use wiki::{handle_lwiki, handle_swiki};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Recieve a 'Pong' from the bot!")]
    Ping,
    #[command(description = "Tell the bot to say anything you type after /echo")]
    Echo(String),
    #[command(description = "get lyrics of a song, in this format \"artist - song\"")]
    Lyrics(String),
    #[command(description = "Random Quote")]
    Quote,
    #[command(description = "Search a gif photo")]
    Gif(String),
    #[command(description = "random photo/gif from tumbler")]
    Tumb,
    #[command(description = "search and get random video")]
    Video(String),
    #[command(description = "Search a full wiki")]
    Lwiki(String),
    #[command(description = "Search a short wiki")]
    Swiki(String),
    #[command(description = "translate english to Portoguese")]
    Pr(String),
    #[command(description = "translate Portoguese to English")]
    En(String),
    #[command(description = "display this text.")]
    Help,
}

#[allow(unreachable_patterns)]
pub async fn answer(cx: UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Echo(s) => cx.answer_str(s).await?,
        Command::Ping => cx.answer_str("Pong!").await?,
        Command::Gif(s) => handle_gif(cx, s).await?,
        Command::Lyrics(s) => handle_lyrics(cx, s).await?,
        Command::Quote => handle_quote(cx).await?,
        Command::Pr(s) => handle_pr(cx, s).await?,
        Command::En(s) => handle_en(cx, s).await?,
        Command::Tumb => handle_tumbler(cx).await?,
        Command::Lwiki(s) => handle_lwiki(cx, s).await?,
        Command::Swiki(s) => handle_swiki(cx, s).await?,
        Command::Video(s) => handle_video(cx, s).await?,
    };

    Ok(())
}

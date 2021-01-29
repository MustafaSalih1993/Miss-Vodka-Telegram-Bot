use crate::commands::common::make_request;
use serde_json::Value;
use std::env;
use teloxide::{
    prelude::{ResponseResult, UpdateWithCx},
    requests::RequestWithFile,
    types::{InputFile, Message},
};

pub async fn handle_photo(cx: UpdateWithCx<Message>, s: String) -> ResponseResult<Message> {
    if s.is_empty() {
        cx.answer_str("Consider Giving The Photo A Name You Fucking Asshole!")
            .await
    } else {
        let photo_url = get_photo(s).await;
        if photo_url.is_none() {
            cx.answer_str("Some shit happend, try again!").await
        } else {
            cx.answer_photo(InputFile::Url(photo_url.unwrap()))
                .send()
                .await
                .unwrap()
        }
    }
}

async fn get_photo(s: String) -> Option<String> {
    let key = env::var("UNSPLASH_KEY").unwrap();
    let url = format!(
        "https://api.unsplash.com/photos/random?client_id={}&query={}",
        key, s
    );

    let resp: Value = match make_request(url).await {
        Some(s) => s,
        None => return None,
    };

    let photo = resp.as_object().unwrap()["urls"].as_object().unwrap()["raw"]
        .as_str()
        .unwrap();

    Some(photo.trim_matches('"').to_string())
}

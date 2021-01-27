use crate::commands::common::make_request;
use std::env;
use teloxide::{prelude::*, requests::RequestWithFile, types::InputFile};

pub async fn handle_gif(cx: UpdateWithCx<Message>, s: String) -> ResponseResult<Message> {
    if s.is_empty() {
        cx.answer_str("Consider Giving The Photo A Name You Fucking Asshole!")
            .await
    } else {
        let gif_url = get_gif(s).await;
        if gif_url.is_none() {
            cx.answer_str("Some shit happend, try again!").await
        } else {
            cx.answer_animation(InputFile::Url(gif_url.unwrap()))
                .send()
                .await
                .unwrap()
        }
    }
}

async fn get_gif(txt: String) -> Option<String> {
    let key = env::var("TENOR_KEY").unwrap();
    // need somthing better
    let txt = txt
        .trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("%20");

    println!("{:?}", txt);
    let url = format!("https://api.tenor.com/v1/random?q={}&key={}&contentfilter=off&limit=1&media_filter=minimal",txt,key);

    let response = match make_request(url).await {
        Some(val) => val,
        None => return None,
    };

    if response["results"].as_array().unwrap().is_empty() {
        return Some(String::from("Not Found, Try Search Somthing Else!"));
    }
    let target = &response["results"][0]["media"][0]["gif"]["url"];

    Some(target.to_string().trim_matches('"').to_string())
}

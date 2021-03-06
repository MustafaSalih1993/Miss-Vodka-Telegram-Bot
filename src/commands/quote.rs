use crate::commands::common::make_request;
use serde_json::Value;
use teloxide::prelude::*;
use teloxide::utils::html;

pub async fn handle_quote(cx: UpdateWithCx<Message>) -> ResponseResult<Message> {
    let quote_data = get_random_quote().await;
    if quote_data.is_none() {
        cx.answer_str("something wrong, try somthing else").await
    } else {
        cx.answer(quote_data.unwrap())
            .parse_mode(teloxide::types::ParseMode::HTML)
            .send()
            .await
    }
}

async fn get_random_quote() -> Option<String> {
    let url = format!("https://quote-garden.herokuapp.com/api/v3/quotes/random");

    let res: Value = match make_request(url).await {
        Some(val) => val,
        None => return None,
    };

    let author = &res["data"].as_array().unwrap()[0]["quoteAuthor"]
        .to_string()
        .trim_matches('"')
        .to_string();
    let text = &res["data"].as_array().unwrap()[0]["quoteText"].to_string();

    Some(format!(
        "{} \n ( {} )",
        html::code_block(text),
        html::bold(&html::underline(author))
    ))
}
